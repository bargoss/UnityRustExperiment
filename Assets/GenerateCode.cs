﻿using System;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using UnityEditor;
using UnityEngine;
using Random = UnityEngine.Random;

namespace DefaultNamespace
{
    public class GenerateCode : MonoBehaviour
    {
        private void Awake()
        {
            var code = CodeGenerator.GenerateCode("mandelbrot", "add_extern", typeof(int), new[] {typeof(int), typeof(int)});
            // full path:
            var path = Path.Combine(Application.dataPath, "Scripts", "Generated", "GeneratedCode.cs");
            print("writing to: " + path);
            File.WriteAllText(path, code);
        }
    }
    
    public class LibraryCall
    {
        public static T Invoke<T, T2>(IntPtr library, params object[] pars)
        {
            IntPtr funcPtr = GetProcAddress(library, typeof(T2).Name);
            if (funcPtr == IntPtr.Zero)
            {
                Debug.LogWarning("Could not gain reference to method address.");
                return default(T);
            }
 
            var func = Marshal.GetDelegateForFunctionPointer(GetProcAddress(library, typeof(T2).Name), typeof(T2));
            return (T)func.DynamicInvoke(pars);
        }
 
        public static void Invoke<T>(IntPtr library, params object[] pars)
        {
            IntPtr funcPtr = GetProcAddress(library, typeof(T).Name);
            if (funcPtr == IntPtr.Zero)
            {
                Debug.LogWarning("Could not gain reference to method address.");
                return;
            }
 
            var func = Marshal.GetDelegateForFunctionPointer(funcPtr, typeof(T));
            func.DynamicInvoke(pars);
        }
 
        [DllImport("kernel32", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        public static extern bool FreeLibrary(IntPtr hModule);
 
        [DllImport("kernel32", SetLastError = true, CharSet = CharSet.Unicode)]
        public static extern IntPtr LoadLibrary(string lpFileName);
 
        [DllImport("kernel32")]
        public static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);    
    }
    
    
    #if UNITY_EDITOR
    public class AddExtern
    {
        [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
        public static extern int add_extern(int a, int b);

        public static int Call(int a, int b)
        {
            return add_extern(a, b);
        }
    }
    #else
    public class AddExtern
    {
        delegate int add_extern(int a, int b);
        
        public static int Call(int a, int b)
        {
            var lib = LibraryCall.LoadLibrary("mandelbrot");
            var result = LibraryCall.Invoke<int, add_extern>(lib, a, b);
            LibraryCall.FreeLibrary(lib);
            return result;
        }
    }
    #endif
    
    // code generator that returns code like above that takes in the delegate and returns the result
    
    public class CodeGenerator
    {
        public static string GenerateCode(string dllName, string methodName, Type returnType, Type[] parameterTypes)
        {
            var code = new StringBuilder();
            code.AppendLine("#if UNITY_EDITOR");
            code.AppendLine($"public class {methodName}");
            code.AppendLine("{");
            code.AppendLine($"[DllImport(\"__Internal\", CallingConvention = CallingConvention.Cdecl)]");
            code.AppendLine($"public static extern {returnType.Name} {methodName}({string.Join(", ", parameterTypes.Select(x => $"{x.Name} {x.Name.ToLower() + Random.Range(0,99999)}"))});");
            code.AppendLine($"public static {returnType.Name} Call({string.Join(", ", parameterTypes.Select(x => $"{x.Name} {x.Name.ToLower() + Random.Range(0,99999)}"))})");
            code.AppendLine("{");
            code.AppendLine($"return {methodName}_extern({string.Join(", ", parameterTypes.Select(x => $"{x.Name.ToLower()}"))});");
            code.AppendLine("}");
            code.AppendLine("}");
            code.AppendLine("#else");
            code.AppendLine($"public class {methodName}");
            code.AppendLine("{");
            code.AppendLine($"delegate {returnType.Name} {methodName}({string.Join(", ", parameterTypes.Select(x => $"{x.Name} {x.Name.ToLower()}"))});");
            code.AppendLine($"public static {returnType.Name} Call({string.Join(", ", parameterTypes.Select(x => $"{x.Name} {x.Name.ToLower()}"))})");
            code.AppendLine("{");
            code.AppendLine($"var lib = LibraryCall.LoadLibrary(\"{dllName}\");");
            code.AppendLine($"var result = LibraryCall.Invoke<{returnType.Name}, {methodName}>(lib, {string.Join(", ", parameterTypes.Select(x => $"{x.Name.ToLower()}"))});");
            code.AppendLine($"LibraryCall.FreeLibrary(lib);");
            code.AppendLine("return result;");
            code.AppendLine("}");
            code.AppendLine("}");
            code.AppendLine("#endif");
            return code.ToString();
        }
    }
}
