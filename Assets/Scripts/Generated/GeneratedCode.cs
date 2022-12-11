
using System;
using System.Runtime.InteropServices;
public class DLLInterface
{
#if UNITY_EDITOR
    private static IntPtr lib;
    public static void Init()
    {
        lib = LibraryCall.LoadLibrary("mandelbrot");
    }
    public static void Cleanup()
    {
        LibraryCall.FreeLibrary(lib);
    }
#else
    public static void Init(){}
    public static void Cleanup(){}
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

    [DllImport("mandelbrot")]
    public static extern IntPtr create_game(Int32 a);
    public static IntPtr CreateGameNative(Int32 a)
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

    [DllImport("mandelbrot")]
    public static extern void update_game(IntPtr a);
    public static void UpdateGameNative(IntPtr a)
    {
        update_game(a);
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

    [DllImport("mandelbrot")]
    public static extern IntPtr get_bubble_positions(IntPtr a);
    public static IntPtr GetBubblePositionsNative(IntPtr a)
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

    [DllImport("mandelbrot")]
    public static extern void apply_bubble_push(IntPtr a, Single b, Single c, Single d);
    public static void ApplyBubblePush(IntPtr a, Single b, Single c, Single d)
    {
        apply_bubble_push(a, b, c, d);
    }
#endif

}