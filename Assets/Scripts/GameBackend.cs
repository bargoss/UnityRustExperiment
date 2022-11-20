using System;
using System.Runtime.InteropServices;
using Generated;
using UnityEngine;

namespace DefaultNamespace
{
    public class GameBackend : MonoBehaviour
    {
        public static GameBackend Instance;
        //public int BubbleCount = 10000;
        //private IntPtr game;
        
        public void Awake()
        {
            DLLInterface.Init();
            Instance = this;
        }
        public IntPtr CreateGame(int bubbleCount)
        {
            return DLLInterface.CreateGameNative(bubbleCount);
        }
        public void ApplyBubblePush(IntPtr game,Vector3 pos)
        {
            //Native.Invoke<apply_bubble_push>(nativeLibraryPtr, game, pos.x, pos.y, pos.z);
            DLLInterface.ApplyBubblePush(game, pos.x, pos.y, pos.z);
        }
        public void UpdateGame(IntPtr game)
        {
            //Native.Invoke<update_game>(nativeLibraryPtr, game);
            DLLInterface.UpdateGameNative(game);
        }
        public Vector3[] GetBubblePositions(IntPtr game, int bubbleCount)
        {
            //var ptr = Native.Invoke<IntPtr, get_bubble_positions>(nativeLibraryPtr, game);
            var ptr = DLLInterface.GetBubblePositionsNative(game);
            var floatArray = new float[bubbleCount * 3];
            Marshal.Copy(ptr, floatArray, 0, bubbleCount * 3);
            var vecArray = new Vector3[bubbleCount];
            for (var i = 0; i < bubbleCount; i++)
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