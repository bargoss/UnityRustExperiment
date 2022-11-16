using System;
using System.Runtime.InteropServices;
using UnityEngine;

namespace DefaultNamespace
{
    public class GameBackend : MonoBehaviour
    {
        public int BubbleCount = 10000;
        private IntPtr game;
        
        public void Awake()
        {
            DLLInterface.Init();
            game = DLLInterface.CreateGameNative(BubbleCount);
        }
        public void ApplyBubblePush(Vector3 pos)
        {
            //Native.Invoke<apply_bubble_push>(nativeLibraryPtr, game, pos.x, pos.y, pos.z);
            DLLInterface.ApplyBubblePush(game, pos.x, pos.y, pos.z);
        }
        public void UpdateGame(IntPtr game)
        {
            //Native.Invoke<update_game>(nativeLibraryPtr, game);
            DLLInterface.UpdateGameNative(game);
        }
        public Vector3[] GetBubblePositions(IntPtr game)
        {
            //var ptr = Native.Invoke<IntPtr, get_bubble_positions>(nativeLibraryPtr, game);
            var ptr = DLLInterface.GetBubblePositionsNative(game);
            var floatArray = new float[BubbleCount * 3];
            Marshal.Copy(ptr, floatArray, 0, BubbleCount * 3);
            var vecArray = new Vector3[BubbleCount];
            for (var i = 0; i < BubbleCount; i++)
            {
                vecArray[i] = new Vector3(floatArray[i * 3], floatArray[i * 3 + 1], floatArray[i * 3 + 2]);
            }
            return vecArray;
        }
        

        void OnApplicationQuit()
        {
            DLLInterface.Cleanup();
        }
    }
}