using Bubbles;
using UnityEditor;
using UnityEngine;

namespace Editor
{
    public class Experiments
    {
        // make a menu item
        [MenuItem("Experiments/ArrayInteropTest")]
        public static void ArrayInteropTest()
        {
            Debug.Log("add result: " + Interop.add_extern(2, 4));
            
            // create a native array
            var nativeArray = new NativeArray<float>(16);
            // set some values
            for (uint i = 0; i < 16; i++)
            {
                nativeArray.SetElement(i, i*2);
            }
            // get some values
            for (uint i = 0; i < 16; i++)
            {
                Debug.Log(nativeArray.GetElement(i));
            }
            // dispose of the native array
            nativeArray.Dispose();
        }
    }
}