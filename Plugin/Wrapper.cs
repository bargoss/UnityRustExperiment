// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
#if UNITY_2018_1_OR_NEWER
using Unity.Collections.LowLevel.Unsafe;
using Unity.Collections;
#endif
using Bubbles;
#pragma warning restore 0105

namespace Bubbles
{
    public static partial class Interop
    {
        #if UNITY_EDITOR
public const string NativeLib = "game_4095061727";
#else
public const string NativeLib = "mandelbrot";
#endif

        static Interop()
        {
        }


        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "create_game")]
        public static extern GameExt create_game();

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "update_game")]
        public static extern void update_game(GameExt game);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "get_bubble_positions")]
        public static extern IntPtr get_bubble_positions(GameExt game);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "apply_bubble_push")]
        public static extern void apply_bubble_push(GameExt game, float x, float y, float z);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "set_push_position")]
        public static extern void set_push_position(GameExt game, float x, float y, float z);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "create_bubble")]
        public static extern uint create_bubble(GameExt game, float x, float y, float z, float radius, float target_distance);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "destroy_bubble")]
        public static extern void destroy_bubble(GameExt game, uint bubble_id);

    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct GameExt
    {
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        public IntPtr ptr;
    }



    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

}
