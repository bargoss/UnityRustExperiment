using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using DefaultNamespace;
using UnityEngine;

public class RustDLL : MonoBehaviour
{
    public int BubbleCount = 10000;
    private IntPtr game;
    
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

        game = GameBackend.Instance.CreateGame(BubbleCount);
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
        GameBackend.Instance.ApplyBubblePush(game, mousePos);
        
        float executionTime = MeasureExecutionTime(() =>
        {
            GameBackend.Instance.UpdateGame(game);
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
        var positions = GameBackend.Instance.GetBubblePositions(game, BubbleCount);
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