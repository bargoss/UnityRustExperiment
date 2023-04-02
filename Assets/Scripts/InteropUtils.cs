using System;
using System.Runtime.InteropServices;
using Bubbles;

public class NativeArray<T> : IDisposable where T : struct
{
    public IntPtr Data;
    public uint Size;

    public NativeArray(uint size)
    {
        Size = size;
        Data = Allocate(size, (uint)Marshal.SizeOf<T>());
    }

    public void Dispose()
    {
        Deallocate(Data, Size, (uint)Marshal.SizeOf<T>());
    }

    public T GetElement(uint index)
    {
        if (index >= Size)
        {
            throw new ArgumentOutOfRangeException(nameof(index));
        }

        IntPtr elementPtr = IntPtr.Add(Data, (int)(index * (uint)Marshal.SizeOf<T>()));
        return Marshal.PtrToStructure<T>(elementPtr);
    }

    public void SetElement(uint index, T value)
    {
        if (index >= Size)
        {
            throw new ArgumentOutOfRangeException(nameof(index));
        }

        IntPtr elementPtr = IntPtr.Add(Data, (int)(index * (uint)Marshal.SizeOf<T>()));
        Marshal.StructureToPtr(value, elementPtr, false);
    }

    private IntPtr Allocate(uint size, uint elem_size)
    {
        return Interop.allocate_native_array(size, elem_size);
    }

    private void Deallocate(IntPtr data, uint size, uint elem_size)
    {
        Interop.deallocate_native_array(data, size, elem_size);
    }
    
}