using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class Experiments : MonoBehaviour
{
    private IntPtr game;
    void Start()
    {
        var res = RustPlugin.add_extern(5, 8);
        
        // spawn that many cubes
        for (int i = 0; i < res; i++)
        {
            var cube = GameObject.CreatePrimitive(PrimitiveType.Cube);
            cube.transform.localScale = Vector3.one * 0.2f;
            cube.transform.position = new Vector3(i * 0.5f, 0, 0);
        }
        Debug.Log("result is: " + res);



        game = RustPlugin.create_game();
        RustPlugin.start_game(game);
    }
    
    int MeasureExecutionTime(Action action)
    {
        var watch = System.Diagnostics.Stopwatch.StartNew();
        action();
        watch.Stop();
        return (int)watch.ElapsedMilliseconds;
    }
    
    int measureCount = 0;
    private float msSum = 0;
    void Update()
    {
        float executionTime = MeasureExecutionTime(() =>
        {
            RustPlugin.update_game(game);
        });
        msSum += executionTime;
        measureCount++;
        
        Debug.Log("Average execution time: " + msSum / measureCount);
        
        var floats = RustPlugin.GetBubblePositions(game);
        Vector3[] positions = new Vector3[floats.Length / 3];
        for (int i = 0; i < floats.Length; i += 3)
        {
            positions[i / 3] = new Vector3(floats[i], floats[i + 1], floats[i + 2]);
        }
        
        // display the bubbles with debug drawrays
        for (int i = 0; i < positions.Length; i++)
        {
            Debug.DrawRay(positions[i], Vector3.forward, Color.red, 0);
        }
    }
}


public class RustPlugin{
#if UNITY_EDITOR
    [DllImport("mandelbrot")]
#else
    [DllImport("__Internal")]
#endif
    public static extern int add_extern(int a, int b);

    
    
    // rust side:
    /*
    #[no_mangle]
    pub extern "C" fn get_float_array(array_id:i32) -> *mut f32 {
        let mut float_array = [0.0; 5];
        float_array[0] = 1.0;
        float_array[1] = 2.0;
        float_array[2] = 3.0;
        float_array[3] = 4.0;
        float_array[4] = 5.0;
        float_array.as_mut_ptr()
    }
    */
    
    // unity side:
#if UNITY_EDITOR
    [DllImport("mandelbrot", CallingConvention = CallingConvention.Cdecl)]
#else
    [DllImport("__Internal", CallingConvention = CallingConvention.Cdecl)]
#endif   
    public static extern IntPtr get_float_array();
    
    public static float[] GetFloatArray()
    {
        var ptr = get_float_array();
        var floatArray = new float[2000*3];
        Marshal.Copy(ptr, floatArray, 0, floatArray.Length);
        return floatArray;
    }

    
#if UNITY_EDITOR
    [DllImport("mandelbrot")]
#else
    [DllImport("__Internal")]
#endif   
    public static extern float get_float_array_value(int array_id, int index);
    
    
#if UNITY_EDITOR
    [DllImport("mandelbrot")]
#else
    [DllImport("__Internal")]
#endif   
    public static extern int get_int_array_value(int array_id, int index);
    
    #if UNITY_EDITOR
        [DllImport("mandelbrot")]
    #else
        [DllImport("__Internal")]
    #endif
    public static extern IntPtr create_game();

    
    #if UNITY_EDITOR
        [DllImport("mandelbrot")]
#else
            [DllImport("__Internal")]
    #endif
    public static extern void start_game(IntPtr game);
    
    #if UNITY_EDITOR
        [DllImport("mandelbrot")]
    #else
        [DllImport("__Internal")]
    #endif
    public static extern void update_game(IntPtr game);
    
    #if UNITY_EDITOR
        [DllImport("mandelbrot")]
    #else
        [DllImport("__Internal")]
    #endif
    public static extern IntPtr get_bubble_positions(IntPtr game);
    
    public static float[] GetBubblePositions(IntPtr game)
    {
        var ptr = get_bubble_positions(game);
        var floatArray = new float[2000 * 3];
        Marshal.Copy(ptr, floatArray, 0, 2000 * 3);
        return floatArray;
    }
}

/* 
 * Native dll invocation helper by Francis R. Griffiths-Keam
 * www.runningdimensions.com/blog
 */