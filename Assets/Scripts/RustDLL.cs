using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using Bubbles;
using DefaultNamespace;
using UnityEngine;

public class RustDLL : MonoBehaviour
{
    public int BubbleCount = 10000;
    private GameExt game;
    
    private void Start()
    {
        // init buffers
        var neededSpace = BubbleCount;
        while (neededSpace > 0)
        {
            var taking = Mathf.Min(neededSpace, 1000);
            _matrixBuffers.Add(new Matrix4x4[taking]);
            neededSpace -= taking;
        }

        game = Interop.create_game(BubbleCount);
    }

    private float msSum = 0;
    private int measureCount = 0;
    
    void Update()
    {
        var mousePos = GetMouseWorldPos();
        Debug.DrawRay(mousePos, Vector3.up, Color.blue);
        Debug.DrawRay(mousePos, Vector3.right, Color.blue);
        Debug.DrawRay(mousePos, Vector3.forward, Color.blue);
        
        //DLLInterface.ApplyBubblePush(game,mousePos);
        Interop.apply_bubble_push(game, mousePos.x, mousePos.y, mousePos.z);
        Interop.apply_bubble_push(game, mousePos.x, mousePos.y, mousePos.z);
        Interop.apply_bubble_push(game, mousePos.x, mousePos.y, mousePos.z);
        
        float executionTime = MeasureExecutionTime(() =>
        {
            Interop.update_game(game);
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
        var positionsPtr = Interop.get_bubble_positions(game);
        var positionFloats = new float[BubbleCount * 3];
        Marshal.Copy(positionsPtr, positionFloats, 0, BubbleCount * 3);
        var positions = new Vector3[BubbleCount];
        for (int i = 0; i < BubbleCount; i++)
        {
            positions[i] = new Vector3(positionFloats[i * 3], positionFloats[i * 3 + 1], positionFloats[i * 3 + 2]);
        }
     
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
}