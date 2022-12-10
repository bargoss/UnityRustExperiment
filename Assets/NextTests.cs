using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using NextModule;
using UnityEngine;

public class NextTests : MonoBehaviour
{
    private IntPtr _moduleHandle;

    // Start is called before the first frame update
    void Start()
    {
        
        // Initialize NextModule Kernel
        print("NextModuleAPI.InitializeEngine(): " + NextModuleAPI.InitializeEngine());
        print(NextModuleAPI.GetLog());

        // Create Handle Pointer
        IntPtr mHandle = IntPtr.Zero;

        // Load Managed Module From Memory
        //IntPtr NativeModulePtr = GCHandle.Alloc(NativeModule, GCHandleType.Pinned).AddrOfPinnedObject();
        //NextModuleAPI.LoadNativeModuleFromStream(NativeModulePtr, NativeModule.Length, ref mHandle);
        var pathToAssets = Application.dataPath;
        var pathToModule = pathToAssets + "/../RuntimePlugins/" + "mandelbrot.dll";
        print("Loading module from: " + pathToModule);
        var bytes = System.IO.File.ReadAllBytes(pathToModule);
        var pinnedBytes = GCHandle.Alloc(bytes, GCHandleType.Pinned).AddrOfPinnedObject();
        print("NextModuleAPI.LoadNativeModuleFromStream(): " + NextModuleAPI.LoadNativeModuleFromStream(pinnedBytes, bytes.Length, ref mHandle));
        print(NextModuleAPI.GetLog());
        
        print("NextModuleAPI.ActivateModule(): " + NextModuleAPI.ActivateModule(mHandle));
        print(NextModuleAPI.GetLog());
        return;
        
        // Activate Module
        NextModuleAPI.ActivateModule(mHandle);

        // Set Module Config
        NextModuleAPI.SetModuleConfig(mHandle, "Application-Name",
            "C#-Native-App", NextModuleAPI.NMDataType.dt_string);

        // Do Something And Wait for half sec
        System.Threading.Thread.Sleep(500);

        // Unload the Module
        NextModuleAPI.UnloadNativeModule(mHandle);

        // Release the Module
        NextModuleAPI.ReleaseNativeModule(mHandle);

        // Display Log
        Console.WriteLine("\nLog Output :\n=====================");
        Console.WriteLine(NextModuleAPI.GetLog());

        // End
        Console.ReadKey();
    }

    // Update is called once per frame
    void Update()
    {
        
    }
    
    // on application quit
    private void OnApplicationQuit()
    {
        // Unload the Module
        NextModuleAPI.UnloadNativeModule(_moduleHandle);
        Console.WriteLine(NextModuleAPI.GetLog());

        // Release the Module
        NextModuleAPI.ReleaseNativeModule(_moduleHandle);
        Console.WriteLine(NextModuleAPI.GetLog());
    }
}
