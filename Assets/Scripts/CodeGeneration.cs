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

        var code = 
            CodeGenerator.GenerateHeader() +  
            CodeGenerator.GenerateDllCall("mandelbrot", "AddExtern","add_extern", typeof(int),new[] { typeof(int), typeof(int) }) +
            CodeGenerator.GenerateFooter();
        
        
        
        var path = Path.Combine(Application.dataPath, "Scripts", "Generated", "GeneratedCode.cs");
        File.WriteAllText(path, code);

        Debug.Log("writing to: " + path);
        Debug.Log("code generation ended");
    }
}
public class CodeGenerator
{
    public static string GenerateHeader()
    {
        var code = @"
using System;
using System.Runtime.InteropServices;
public class DLLInterface
{
";
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

        string code = @"

#if UNITY_EDITOR
    delegate replace_returnType replace_methodName(replace_paramsStrWithTypes);
    public static replace_returnType replace_Call(replace_paramsStrWithTypes)
    {
        var lib = LibraryCall.LoadLibrary(""replace_dllName"");
        var result = LibraryCall.Invoke<replace_returnType, replace_methodName>(lib, replace_paramsStr);
        LibraryCall.FreeLibrary(lib);
        return result;
    }
#else
    [DllImport(""__Internal"", CallingConvention = CallingConvention.Cdecl)]
    public static extern replace_returnType replace_methodName(replace_paramsStrWithTypes);
    public static replace_returnType replace_Call(replace_paramsStr)
    {
        return replace_methodName(replace_paramsStr);
    }
#endif

"
            .Replace("replace_returnType", returnType.Name)
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