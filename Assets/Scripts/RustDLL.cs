using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class RustDLL : MonoBehaviour
{
    static IntPtr nativeLibraryPtr;
 
    //delegate int MultiplyFloat(float number, float multiplyBy);
    //delegate void DoSomething(string words);
    delegate IntPtr get_int_array_ptr();
    delegate IntPtr create_game(int bubble_count);
    delegate void update_game(IntPtr game);
    delegate IntPtr get_bubble_positions(IntPtr game);
    //pub extern "C" fn get_float_array_value(array_id:i32, index:i32) -> f32
    delegate float get_float_array_value(int array_id, int index);
    delegate void apply_bubble_push(IntPtr game, float x, float y, float z);

    public void ApplyBubblePush(Vector3 pos)
    {
        Native.Invoke<apply_bubble_push>(nativeLibraryPtr, game, pos.x, pos.y, pos.z);
    }
    public IntPtr CreateGame(int bubbleCount)
    {
        return Native.Invoke<IntPtr, create_game>(nativeLibraryPtr, bubbleCount);
    }
    public void UpdateGame(IntPtr game)
    {
        Native.Invoke<update_game>(nativeLibraryPtr, game);
    }
    public Vector3[] GetBubblePositions(IntPtr game)
    {
        var ptr = Native.Invoke<IntPtr, get_bubble_positions>(nativeLibraryPtr, game);
        var floatArray = new float[bubbleCount * 3];
        Marshal.Copy(ptr, floatArray, 0, bubbleCount * 3);
        var vecArray = new Vector3[bubbleCount];
        for (var i = 0; i < bubbleCount; i++)
        {
            vecArray[i] = new Vector3(floatArray[i * 3], floatArray[i * 3 + 1], floatArray[i * 3 + 2]);
        }
        return vecArray;
    }
    public float GetFloatArrayValue(int array_id, int index)
    {
        return Native.Invoke<float, get_float_array_value>(nativeLibraryPtr, array_id, index);
    }
    
 
 
    void Awake()
    {
        if (nativeLibraryPtr != IntPtr.Zero) return;
 
        nativeLibraryPtr = Native.LoadLibrary("mandelbrot");
        if (nativeLibraryPtr == IntPtr.Zero)
        {
            Debug.LogError("Failed to load native library");
        }


        // init matrix buffers
        var neededSpace = bubbleCount;
        while (neededSpace > 0)
        {
            var taking = Mathf.Min(neededSpace, 1000);
            _matrixBuffers.Add(new Matrix4x4[taking]);
            neededSpace -= taking;
        }
    }

    private IntPtr game;
    private void Start()
    {
        game = CreateGame(bubbleCount);
        print("0: " + GetFloatArrayValue(0, 0));
        print("1: " + GetFloatArrayValue(0, 1));
        print("2: " + GetFloatArrayValue(0, 2));
        print("3: " + GetFloatArrayValue(0, 3));
    }

    private float msSum = 0;
    private int measureCount = 0;

    public int bubbleCount = 100;
    
    void Update()
    {
        var mousePos = GetMouseWorldPos();
        Debug.DrawRay(mousePos, Vector3.up, Color.blue);
        Debug.DrawRay(mousePos, Vector3.right, Color.blue);
        Debug.DrawRay(mousePos, Vector3.forward, Color.blue);
        
        ApplyBubblePush(mousePos);
        
        float executionTime = MeasureExecutionTime(() =>
        {
            UpdateGame(game);
        });
        msSum += executionTime;
        measureCount++;

        //Debug.Log("Average execution time: " + msSum / measureCount);
        //var positions = GetBubblePositions(game);
        //for (int i = 0; i < positions.Length; i++)
        //{
        //    Debug.DrawRay(positions[i], Vector3.forward, Color.red, 0);
        //}
        DrawBubblesNice();
    }

    public Mesh BubbleMesh;
    public Material BubbleMaterial;
    private List<Matrix4x4[]> _matrixBuffers = new List<Matrix4x4[]>();
    public void DrawBubblesNice()
    {
        var positions = GetBubblePositions(game);
        for (int i = 0; i < positions.Length; i++)
        {
            int bufferIndex = i / 1000;
            int indexInBuffer = i % 1000;
            _matrixBuffers[bufferIndex][indexInBuffer] = Matrix4x4.TRS(positions[i], Quaternion.identity, Vector3.one * 3);
        }
        // use default sphere
        for (int i = 0; i < _matrixBuffers.Count; i++)
        {
            Graphics.DrawMeshInstanced(BubbleMesh, 0, BubbleMaterial, _matrixBuffers[i]);
        }
    }

    Vector3 GetMouseWorldPos()
    {
        var plane = new Plane(Vector3.forward, Vector3.zero);
        var ray = Camera.main.ScreenPointToRay(Input.mousePosition);
        plane.Raycast(ray, out var distance);
        return ray.GetPoint(distance);
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