using System.IO;
using System.Linq;
using System.Text;
using UnityEditor;
using UnityEngine;
using Random = UnityEngine.Random;


public class GenerateCode : MonoBehaviour
{
    private void Awake()
    {
        var code = CodeGenerator.GenerateDllCall("mandelbrot", "AddExtern","add_extern", typeof(int),
            new[] { typeof(int), typeof(int) });
        // full path:
        var path = Path.Combine(Application.dataPath, "Scripts", "Generated", "GeneratedCode.cs");
        print("writing to: " + path);
        File.WriteAllText(path, code);
    }
}


public class DLLInterface
{
#if UNITY_EDITOR
    delegate int add_extern(int a, int b);

    public static int Call(int a, int b)
    {
        var lib = LibraryCall.LoadLibrary("mandelbrot");
        var result = LibraryCall.Invoke<int, add_extern>(lib, a, b);
        LibraryCall.FreeLibrary(lib);
        return result;
    }
#else
        [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
        public static extern int add_extern(int a, int b);
        public static int Call(int a, int b)
        {
            return add_extern(a, b);
        }
#endif
}

// code generator that returns code like above that takes in the delegate and returns the result



