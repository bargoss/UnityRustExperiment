using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class RustDLL : MonoBehaviour
{
    static IntPtr nativeLibraryPtr;
 
    //delegate int MultiplyFloat(float number, float multiplyBy);
    //delegate void DoSomething(string words);
    delegate IntPtr create_game();
    delegate void start_game(IntPtr game);
    delegate void update_game(IntPtr game);
    delegate IntPtr get_bubble_positions(IntPtr game);

    public IntPtr CreateGame()
    {
        return Native.Invoke<IntPtr, create_game>(nativeLibraryPtr, "Hello, World!");
    }
    public void StartGame(IntPtr game)
    {
        Native.Invoke<IntPtr, start_game>(nativeLibraryPtr, game);
    }
    public void UpdateGame(IntPtr game)
    {
        Native.Invoke<IntPtr, update_game>(nativeLibraryPtr, game);
    }
    public Vector3[] GetBubblePositions(IntPtr game)
    {
        var ptr = Native.Invoke<IntPtr, get_bubble_positions>(nativeLibraryPtr, game);
        var floatArray = new float[500 * 3];
        Marshal.Copy(ptr, floatArray, 0, 500 * 3);
        var vecArray = new Vector3[500];
        for (var i = 0; i < 500; i++)
        {
            vecArray[i] = new Vector3(floatArray[i * 3], floatArray[i * 3 + 1], floatArray[i * 3 + 2]);
        }

        return vecArray;
    }
    
 
 
    void Awake()
    {
        if (nativeLibraryPtr != IntPtr.Zero) return;
 
        nativeLibraryPtr = Native.LoadLibrary("mandelbrot");
        if (nativeLibraryPtr == IntPtr.Zero)
        {
            Debug.LogError("Failed to load native library");
        }
    }

    private IntPtr game;
    private void Start()
    {
        game = CreateGame();
        StartGame(game);
    }

    private float msSum = 0;
    private int measureCount = 0;
    
    void Update()
    {
        float executionTime = MeasureExecutionTime(() =>
        {
            UpdateGame(game);
        });
        msSum += executionTime;
        measureCount++;
        
        Debug.Log("Average execution time: " + msSum / measureCount);
        
        var positions = GetBubblePositions(game);
        // display the bubbles with debug drawrays
        for (int i = 0; i < positions.Length; i++)
        {
            Debug.DrawRay(positions[i], Vector3.forward, Color.red, 0);
        }
    }
    
    int MeasureExecutionTime(Action action)
    {
        var watch = System.Diagnostics.Stopwatch.StartNew();
        action();
        watch.Stop();
        return (int)watch.ElapsedMilliseconds;
    }
 
    void OnApplicationQuit()
    {
        if (nativeLibraryPtr == IntPtr.Zero) return;
 
        Debug.Log(Native.FreeLibrary(nativeLibraryPtr)
            ? "Native library successfully unloaded."
            : "Native library could not be unloaded.");
    }
}