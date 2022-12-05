
abstract class Solver<T1, T2> {
    public T2 PartOne(string[] input) => DoPartOne(Parse(input));
    public T2 PartTwo(string[] input) => DoPartTwo(Parse(input));
  
    public abstract T1 Parse(string[] input);
    public abstract T2 DoPartOne(T1 input);
    public abstract T2 DoPartTwo(T1 input);
}
