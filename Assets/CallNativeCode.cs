using UnityEngine;
using System.Collections;
using System.Runtime.InteropServices;

public class CallNativeCode : MonoBehaviour {

	[DllImport("native")]
	private static extern float add(float x, float y);

	void Update ()
	{
		float x = 3;
		float y = 10;
		
		Debug.LogError("adding " + x  + " and " + y + " in native code equals " + add(x,y));
		
	}
}
