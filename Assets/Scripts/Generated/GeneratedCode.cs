
using System;
using System.Runtime.InteropServices;
public class DLLInterface
{
#if UNITY_EDITOR
 private static IntPtr lib;
 private static void Init()
 {
     lib = LibraryCall.LoadLibrary("mandelbrot");
 }
 private static void Cleanup()
 {
     LibraryCall.FreeLibrary(lib);
 }
#else
private static void Init(){}
private static void Cleanup(){}
#endif


#if UNITY_EDITOR
    delegate IntPtr create_game(Int32 a);
    public static IntPtr CreateGameNative(Int32 a)
    {
        var result = LibraryCall.Invoke<IntPtr, create_game>(lib, a);
        //LibraryCall.FreeLibrary(lib);
        return result;
    }
#else

    [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr create_game(Int32 a);
    public static IntPtr CreateGameNative(a)
    {
        return create_game(a);
    }
#endif


#if UNITY_EDITOR
    delegate void update_game(IntPtr a);
    public static void UpdateGameNative(IntPtr a)
    {
        LibraryCall.Invoke<update_game>(lib, a);
        //LibraryCall.FreeLibrary(lib);
    }
#else

    [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
    public static extern Void update_game(IntPtr a);
    public static Void UpdateGameNative(a)
    {
        return update_game(a);
    }
#endif



#if UNITY_EDITOR
    delegate IntPtr get_bubble_positions(IntPtr a);
    public static IntPtr GetBubblePositionsNative(IntPtr a)
    {
        var result = LibraryCall.Invoke<IntPtr, get_bubble_positions>(lib, a);
        //LibraryCall.FreeLibrary(lib);
        return result;
    }
#else

    [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr get_bubble_positions(IntPtr a);
    public static IntPtr GetBubblePositionsNative(a)
    {
        return get_bubble_positions(a);
    }
#endif


#if UNITY_EDITOR
    delegate void apply_bubble_push(IntPtr a, Single b, Single c, Single d);
    public static void ApplyBubblePush(IntPtr a, Single b, Single c, Single d)
    {
        LibraryCall.Invoke<apply_bubble_push>(lib, a, b, c, d);
        //LibraryCall.FreeLibrary(lib);
    }
#else

    [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
    public static extern Void apply_bubble_push(IntPtr a, Single b, Single c, Single d);
    public static Void ApplyBubblePush(a, b, c, d)
    {
        return apply_bubble_push(a, b, c, d);
    }
#endif

}