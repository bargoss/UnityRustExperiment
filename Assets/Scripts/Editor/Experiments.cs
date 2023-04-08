using Bubbles;
using Unity.Plastic.Newtonsoft.Json;
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
            /*
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
            */
        }

        [MenuItem("Experiments/311523")]
        public static void TestArenaGame()
        {
            var game = Interop.create_game();
            Interop.register_views(game);
            Interop.advance_tick(game);
            Interop.register_views(game);
            Interop.advance_tick(game);
            Interop.register_views(game);
            Interop.advance_tick(game);
            Interop.register_views(game);
            Interop.advance_tick(game);
            Interop.register_views(game);
            Interop.advance_tick(game);
            Interop.register_views(game);
            Interop.advance_tick(game);
            Interop.register_views(game);
            Interop.advance_tick(game);
            Interop.render(game, 0.02f, SphereRenderAction);
            Debug.Log("----");
            Interop.render(game, 0.04f, SphereRenderAction);
            Debug.Log("----");
            Interop.render(game, 0.05f, SphereRenderAction);
            Debug.Log("----");
            
        }
        
        private static void SphereRenderAction(SphereRenderParams x0)
        {
            Debug.Log("SphereRenderAction: " + JsonConvert.SerializeObject(x0));
        }

        struct MyStruct
        {
            // make a fixed size array of size 10
            public int[] myArray;
            
        }
        
        [MenuItem("Experiments/31152321")]
        public static void Stuff()
        {
            int[] assd = new int[3];
        }
    }
}