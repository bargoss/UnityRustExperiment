using System;
using System.Runtime.InteropServices;

#if UNITY_EDITOR
public class add_extern
{
[DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
public static extern Int32 add_extern(Int32 int3246014, Int32 int3228985);
public static Int32 Call(Int32 int32, Int32 int32)
{
return add_extern_extern(int32, int32);
}
}
#else
public class add_extern
{
delegate Int32 add_extern(Int32 int32, Int32 int32);
public static Int32 Call(Int32 int32, Int32 int32)
{
var lib = LibraryCall.LoadLibrary("mandelbrot");
var result = LibraryCall.Invoke<Int32, add_extern>(lib, int32, int32);
LibraryCall.FreeLibrary(lib);
return result;
}
}
#endif
