/*
╔════════════════════════════════════════════════════════════╗
║        __          _                    _       _          ║
║     /\ \ \_____  _| |_  /\/\   ___   __| |_   _| | ___     ║
║    /  \/ / _ \ \/ / __|/    \ / _ \ / _` | | | | |/ _ \    ║
║   / /\  /  __/>  <| |_/ /\/\ \ (_) | (_| | |_| | |  __/    ║
║   \_\ \/ \___/_/\_\\__\/    \/\___/ \__,_|\__,_|_|\___|    ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
**************************************************************
==============================================================
 	      Developed by Hamid.Memar - Copyright 2020
		     	  	< MemarDesign™ LLC. >
==============================================================
1. Don't use this library to build viruses and malewares.
2. This library is licensed under Apache 2.0 License.
3. Read the fucking manual before using this library.
4. If you experienced any issue report it on github repo.
5. Wash your hands for fucksake!
==============================================================
*/

using System;
using System.Runtime.InteropServices;

namespace NextModule
{

	public class NextModuleAPI
	{
		// Enums & Data Types //

		public enum NMDataType
		{
			dt_string, dt_widestring, dt_short,
			dt_integer, dt_long, dt_float,
			dt_double, dt_boolean, dt_pointer,
			dt_null, dt_unknown
		};
		public enum NMModuleState
		{
			st_Loaded, st_Activated,
			st_Deativated, st_Suspended,
			st_Unloaded, st_Released,
			st_Purged, st_Unknown
		};

		// Library Imports

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_InitializeEngine();

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_LoadNativeModule(string moduleFile, ref IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_LoadNativeModuleFromStream(IntPtr rawData, long rawDataSize, ref IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_UnloadNativeModule(IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_ReleaseNativeModule(IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_LoadJITModule(string moduleFile, ref IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_LoadJITModuleFromStream(IntPtr rawData, long rawDataSize, ref IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_UnloadJITModule(IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_ActivateModule(IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_SetModuleData(IntPtr moduleHandle, string dataName, NMDataType dataType, IntPtr pointerToData, long dataSize);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_SetModuleConfig(IntPtr moduleHandle, string configName, string configValue, NMDataType dataType = NMDataType.dt_unknown);

		[DllImport("NextModule.Win64.dll")]
		extern static private int NEXTMODULE_GetModuleState(IntPtr moduleHandle);

		[DllImport("NextModule.Win64.dll")]
		extern static private string NEXTMODULE_GetLoadedModules();

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_SetGlobalData(string dataName, NMDataType dataType, IntPtr pointerToData, long dataSize);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_GetGlobalData(string dataName, ref NMDataType dataType, ref IntPtr pointerToData, ref long dataSize);

		[DllImport("NextModule.Win64.dll")]
		extern static private bool NEXTMODULE_HasError();

		[DllImport("NextModule.Win64.dll")]
		extern static private int NEXTMODULE_GetLastError();

		[DllImport("NextModule.Win64.dll" , CharSet = CharSet.Unicode)]
		extern static private string NEXTMODULE_GetLog();

		[DllImport("NextModule.Win64.dll")]
		extern static private void NEXTMODULE_AddLog(string logMsg);

		[DllImport("NextModule.Win64.dll")]
		extern static private void NEXTMODULE_ClearLog();

		[DllImport("NextModule.Win64.dll")]
		extern static private string NEXTMODULE_About();

		// NextMoudle API //

		static public bool InitializeEngine()
		{
			return NEXTMODULE_InitializeEngine();
		}
		static public bool LoadNativeModule(string moduleFile, ref IntPtr moduleHandle)
		{
			return NEXTMODULE_LoadNativeModule(moduleFile, ref moduleHandle);
		}
		static public bool LoadNativeModuleFromStream(IntPtr rawData, long rawDataSize, ref IntPtr moduleHandle)
		{
			return NEXTMODULE_LoadNativeModuleFromStream(rawData, rawDataSize, ref moduleHandle);
		}
		static public bool UnloadNativeModule(IntPtr moduleHandle)
		{
			return NEXTMODULE_UnloadNativeModule(moduleHandle);
		}
		static public bool ReleaseNativeModule(IntPtr moduleHandle)
		{
			return NEXTMODULE_ReleaseNativeModule(moduleHandle);
		}
		static public bool LoadJITModule(string moduleFile, ref IntPtr moduleHandle)
		{
			return NEXTMODULE_LoadJITModule(moduleFile, ref moduleHandle);
		}
		static public bool LoadJITModuleFromStream(IntPtr rawData, long rawDataSize, ref IntPtr moduleHandle)
		{
			return NEXTMODULE_LoadJITModuleFromStream(rawData, rawDataSize, ref moduleHandle);
		}
		static public bool UnloadJITModule(IntPtr moduleHandle)
		{
			return NEXTMODULE_UnloadJITModule(moduleHandle);
		}
		static public bool ActivateModule(IntPtr moduleHandle)
		{
			return NEXTMODULE_ActivateModule(moduleHandle);
		}
		static public bool SetModuleData(IntPtr moduleHandle, string dataName, NMDataType dataType, IntPtr pointerToData, long dataSize)
		{
			return NEXTMODULE_SetModuleData(moduleHandle, dataName, dataType, pointerToData, dataSize);
		}
		static public bool SetModuleConfig(IntPtr moduleHandle, string configName, string configValue, NMDataType dataType = NMDataType.dt_unknown)
		{
			return NEXTMODULE_SetModuleConfig(moduleHandle, configName, configValue, dataType);
		}
		static public int GetModuleState(IntPtr moduleHandle)
		{
			return NEXTMODULE_GetModuleState(moduleHandle);
		}
		static public string GetLoadedModules()
		{
			return NEXTMODULE_GetLoadedModules();
		}
		static public bool SetGlobalData(string dataName, NMDataType dataType, IntPtr pointerToData, long dataSize)
		{
			return NEXTMODULE_SetGlobalData(dataName, dataType, pointerToData, dataSize);
		}
		static public bool GetGlobalData(string dataName, ref NMDataType dataType, ref IntPtr pointerToData, ref long dataSize)
		{
			return NEXTMODULE_GetGlobalData(dataName, ref dataType, ref pointerToData, ref dataSize);
		}
		static public bool HasError()
		{
			return NEXTMODULE_HasError();
		}
		static public int GetLastError()
		{
			return NEXTMODULE_GetLastError();
		}
		static public string GetLog()
		{
			return System.Text.Encoding.ASCII.GetString(
			   System.Text.Encoding.Unicode.GetBytes(NEXTMODULE_GetLog()));
		}
		static public void AddLog(string logMsg)
		{
			NEXTMODULE_AddLog(logMsg);
		}
		static public void ClearLog()
		{
			NEXTMODULE_ClearLog();
		}
		static public string About()
		{
			return NEXTMODULE_About();
		}
	}

}