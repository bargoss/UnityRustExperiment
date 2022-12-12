using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using Bubbles;
using DefaultNamespace;
using UnityEngine;

public class RustDLL : MonoBehaviour
{
    public const int BubbleCount = 100; //2000;
    public float NeighborForce;
    public float Viscosity;
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


    void HandleBubblePushing()
    {
        // get touches
        var touches = Input.touches;
        for (var i = 0; i < touches.Length; i++)
        {
            var touch = touches[i];
            var worldPos = CameraSpaceToBubblePlaneSpace(touch.position);
            Interop.apply_bubble_push(game, worldPos.x, worldPos.y, worldPos.z);
        }

#if UNITY_EDITOR
        if (Input.GetMouseButton(0))
        {
            var worldPos = CameraSpaceToBubblePlaneSpace(Input.mousePosition);
            Interop.apply_bubble_push(game, worldPos.x, worldPos.y, worldPos.z);
        }
#endif
    }
    
    void FixedUpdate()
    {
        var mousePos = GetMouseWorldPos();
        Debug.DrawRay(mousePos, Vector3.up, Color.blue);
        Debug.DrawRay(mousePos, Vector3.right, Color.blue);
        Debug.DrawRay(mousePos, Vector3.forward, Color.blue);
        
        //DLLInterface.ApplyBubblePush(game,mousePos);
        HandleBubblePushing();

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
        //DrawBubblesNice();
        
    }

    public void Update()
    {
        DrawWithQuads();
    }

    public Mesh BubbleMesh;
    public Material BubbleMaterial;
    private List<Matrix4x4[]> _matrixBuffers = new List<Matrix4x4[]>();
    private float[] _positionFloatBuffer = new float[BubbleCount * 3];
    public void DrawBubblesNice()
    {
        var positionsPtr = Interop.get_bubble_positions(game);
        Marshal.Copy(positionsPtr, _positionFloatBuffer, 0, BubbleCount * 3);
        var positions = new Vector3[BubbleCount];
        for (int i = 0; i < BubbleCount; i++)
        {
            positions[i] = new Vector3(_positionFloatBuffer[i * 3], _positionFloatBuffer[i * 3 + 1], _positionFloatBuffer[i * 3 + 2]);
        }
     
        for (int i = 0; i < positions.Length * 0.1f; i++)
        {
            int bufferIndex = i / 1000;
            int indexInBuffer = i % 1000;
            _matrixBuffers[bufferIndex][indexInBuffer] = Matrix4x4.TRS(positions[i], Quaternion.identity, Vector3.one * 3);
        }

        BubbleMaterial.enableInstancing = true;
        
        // use default sphere
        for (int i = 0; i < _matrixBuffers.Count; i++)
        {
            Graphics.DrawMeshInstanced(BubbleMesh, 0, BubbleMaterial, _matrixBuffers[i]);
        }
    }

    private List<Vector3> _bubbleVertices = new List<Vector3>();
    private List<int> _bubbleIndices = new List<int>();
    private Mesh _mesh;
    public void DrawWithQuads()
    {
        var positionsPtr = Interop.get_bubble_positions(game);
        Marshal.Copy(positionsPtr, _positionFloatBuffer, 0, BubbleCount * 3);
        var positions = new Vector3[BubbleCount];
        for (int i = 0; i < BubbleCount; i++)
        {
            positions[i] = new Vector3(_positionFloatBuffer[i * 3], _positionFloatBuffer[i * 3 + 1], _positionFloatBuffer[i * 3 + 2]);
        }
        
        _bubbleVertices.Clear();
        _bubbleIndices.Clear();
        
        for (int i = 0; i < positions.Length; i++)
        {
            var pos = positions[i];
            var index = _bubbleVertices.Count;
            _bubbleVertices.Add(pos + new Vector3(0, 0, -1f)); //0
            _bubbleVertices.Add(pos + new Vector3(1, 1, 0)); //1
            _bubbleVertices.Add(pos + new Vector3(-1, 1, 0)); //2
            _bubbleVertices.Add(pos + new Vector3(-1, -1, 0)); //3
            _bubbleVertices.Add(pos + new Vector3(1, -1, 0)); //4
            /*
             
            2     1
               0
            3     4
            
             */
            
            _bubbleIndices.Add(index);
            _bubbleIndices.Add(index + 2);
            _bubbleIndices.Add(index + 1);

            _bubbleIndices.Add(index);
            _bubbleIndices.Add(index + 3);
            _bubbleIndices.Add(index + 2);

            _bubbleIndices.Add(index);
            _bubbleIndices.Add(index + 4);
            _bubbleIndices.Add(index + 3);

            _bubbleIndices.Add(index);
            _bubbleIndices.Add(index + 1);
            _bubbleIndices.Add(index + 4);
        }
        
        //_mesh.Clear();
        if (_mesh == null) _mesh = new Mesh();
        _mesh.SetVertices(_bubbleVertices);
        _mesh.SetIndices(_bubbleIndices, MeshTopology.Triangles, 0);
        _mesh.RecalculateNormals();
        Graphics.DrawMesh(_mesh, Matrix4x4.identity, BubbleMaterial, 0);
    }

    Vector3 CameraSpaceToBubblePlaneSpace(Vector2 screenPos)
    {
        var plane = new Plane(Vector3.forward, Vector3.zero);
        var ray = Camera.main.ScreenPointToRay(screenPos);
        plane.Raycast(ray, out var distance);
        return ray.GetPoint(distance);
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