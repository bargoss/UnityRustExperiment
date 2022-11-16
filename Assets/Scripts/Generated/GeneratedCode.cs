
using System;
using System.Runtime.InteropServices;
public class DLLInterface
{


#if UNITY_EDITOR
    private static IntPtr lib = LibraryCall.LoadLibrary("mandelbrot");
    delegate Int32 add_extern(Int32 a, Int32 b);
    public static Int32 AddExtern(Int32 a, Int32 b)
    {
        var result = LibraryCall.Invoke<Int32, add_extern>(lib, a, b);
        LibraryCall.FreeLibrary(lib);
        return result;
    }
#else
    [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
    public static extern Int32 add_extern(Int32 a, Int32 b);
    public static Int32 AddExtern(a, b)
    {
        return add_extern(a, b);
    }
#endif

}