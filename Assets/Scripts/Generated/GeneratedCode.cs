using System;
using System.Runtime.InteropServices;

namespace Generated
{
    public class DLLInterface
    {

        public static void Init(){}
        public static void Cleanup(){}



    
        [DllImport("__Internal")]
        public static extern void create_game(Int32 a);
        public static void CreateGameNative(Int32 a)
        {
            create_game(a);
        }


    

        //[DllImport("__Internal")]
        //public static extern void update_game(IntPtr a);
        //public static void UpdateGameNative(IntPtr a)
        //{
//            update_game(a);
        //}



        //[DllImport("__Internal")]
        //public static extern IntPtr get_bubble_positions(IntPtr a);
        //public static IntPtr GetBubblePositionsNative(IntPtr a)
        //{
        //    return get_bubble_positions(a);
        //}
    
        //[DllImport("__Internal")]
        //public static extern void apply_bubble_push(IntPtr game, float x, float y, float z);
        //public static void ApplyBubblePush(IntPtr game, float x, float y, float z)
        //{
        //    apply_bubble_push(game, x, y, z);
        //}


    }
}