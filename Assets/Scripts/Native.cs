// how to use partial class example:

partial class MyClass
{
    public const string Field = "a";
}

partial class MyClass
{
    public const string Field2 = "b";
}



public class Tests
{
    public void Test()
    {
        var c = new MyClass();
    }
}

