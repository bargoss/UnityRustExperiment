using System;
using System.IO;
using System.Linq;
using System.Text;
using UnityEditor;
using UnityEditor.Build;
using UnityEditor.Build.Reporting;
using UnityEngine;

class CodeGeneration : Editor// : IPreprocessBuildWithReport
{
    [MenuItem("Barans/GenerateCode")]
    public static void Generate()
    {
        Debug.Log("code generation started");

        
        //delegate IntPtr create_game(int bubble_count);
        //delegate void update_game(IntPtr game);
        //delegate IntPtr get_bubble_positions(IntPtr game);
        //delegate void apply_bubble_push(IntPtr game, float x, float y, float z);
        
        var code = 
            CodeGenerator.GenerateHeader("mandelbrot") +
            CodeGenerator.GenerateDllCall("mandelbrot", "CreateGameNative","create_game", typeof(IntPtr),new[] { typeof(int) }) +
            CodeGenerator.GenerateDllCall("mandelbrot", "UpdateGameNative","update_game", typeof(void),new[] { typeof(IntPtr) }) +
            CodeGenerator.GenerateDllCall("mandelbrot", "GetBubblePositionsNative","get_bubble_positions", typeof(IntPtr),new[] { typeof(IntPtr)}) +
            CodeGenerator.GenerateDllCall("mandelbrot", "ApplyBubblePush","apply_bubble_push", typeof(void),new[] { typeof(IntPtr), typeof(float), typeof(float), typeof(float)}) +
            CodeGenerator.GenerateFooter();
        
        
        
        var path = Path.Combine(Application.dataPath, "Scripts", "Generated", "GeneratedCode.cs");
        File.WriteAllText(path, code);

        Debug.Log("writing to: " + path);
        Debug.Log("code generation ended");
    }
}
public class CodeGenerator
{
    public static string GenerateHeader(string dllName)
    {
        var code = @"
using System;
using System.Runtime.InteropServices;
public class DLLInterface
{
#if UNITY_EDITOR
 private static IntPtr lib;
 private static void Init()
 {
     lib = LibraryCall.LoadLibrary(""replace_dllName"");
 }
 private static void Cleanup()
 {
     LibraryCall.FreeLibrary(lib);
 }
#else
private static void Init(){}
private static void Cleanup(){}
#endif
".Replace("replace_dllName", dllName);
        return code;
    }

    public static string GenerateFooter()
    {
        var code = "}";
        return code;
    }

    public static string GenerateDllCall(string dllName, string callName, string methodName, Type returnType, Type[] parameterTypes)
    {
        var parameters = "abcdefghijklmnopqrstuvwxyz";
        var paramsStrWithTypes = string.Join(", ", parameterTypes.Select((t, i) => $"{t.Name} {parameters[i]}"));
        var paramsStr = string.Join(", ", parameterTypes.Select((t, i) => $"{parameters[i]}"));

        string code = "";
        string withReturnType = @"

#if UNITY_EDITOR
    delegate replace_returnType replace_methodName(replace_paramsStrWithTypes);
    public static replace_returnType replace_Call(replace_paramsStrWithTypes)
    {
        var result = LibraryCall.Invoke<replace_returnType, replace_methodName>(lib, replace_paramsStr);
        //LibraryCall.FreeLibrary(lib);
        return result;
    }
#else
";
        string withVoidReturnType = @"
#if UNITY_EDITOR
    delegate void replace_methodName(replace_paramsStrWithTypes);
    public static void replace_Call(replace_paramsStrWithTypes)
    {
        LibraryCall.Invoke<replace_methodName>(lib, replace_paramsStr);
        //LibraryCall.FreeLibrary(lib);
    }
#else
";

        if (returnType == typeof(void))
        {
            code += withVoidReturnType;
        }
        else
        {
            code += withReturnType;
        }
        
        code += @"
    [DllImport(""__Internal"", CallingConvention = CallingConvention.Cdecl)]
    public static extern replace_returnType replace_methodName(replace_paramsStrWithTypes);
    public static replace_returnType replace_Call(replace_paramsStr)
    {
        return replace_methodName(replace_paramsStr);
    }
#endif

";
            code = code.Replace("replace_returnType", returnType.Name)
            .Replace("replace_methodName", methodName)
            .Replace("replace_paramsStrWithTypes", paramsStrWithTypes)
            .Replace("replace_paramsStr", paramsStr)
            .Replace("replace_dllName", dllName)
            .Replace("replace_Call", callName);

        return code;


        //var code = new StringBuilder();
        //code.AppendLine("using System;");
        //code.AppendLine("using System.Runtime.InteropServices;");
        //code.AppendLine("            public class DLLInterface");
        //code.AppendLine("            {");
        //code.AppendLine("#if UNITY_EDITOR");
        //code.AppendLine($"            delegate {returnType.Name} {methodName}({paramsStrWithTypes});");
        //code.AppendLine($"               public static {returnType.Name} Call({paramsStrWithTypes})");
        //code.AppendLine("                {");
        //code.AppendLine("                    var lib = LibraryCall.LoadLibrary(\"" + dllName + "\");");
        //code.AppendLine($"                    var result = LibraryCall.Invoke<{returnType.Name}, {methodName}>(lib, {paramsStr});");
        //code.AppendLine("                    LibraryCall.FreeLibrary(lib);");
        //code.AppendLine("                    return result;");
        //code.AppendLine("                }");
        //code.AppendLine("#else");
        //code.AppendLine("        [DllImport(\"__Internal\", CallingConvention = CallingConvention.Cdecl)]");
        //code.AppendLine("        public static extern int add_extern(int a, int b);");
        //code.AppendLine("        public static int Call(" + paramsStr + ")");
        //code.AppendLine("        {");
        //code.AppendLine("            return add_extern(" + paramsStr + ");");
        //code.AppendLine("        }");
        //code.AppendLine("#endif");
        //code.AppendLine("            }");
        //return code.ToString();
    }
}