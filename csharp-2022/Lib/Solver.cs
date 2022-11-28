
abstract class Solver<T> {
    public int PartOne(string[] input) => DoPartOne(Parse(input));
    public int PartTwo(string[] input) => DoPartTwo(Parse(input));
  
    public abstract T Parse(string[] input);
    public abstract int DoPartOne(T input);
    public abstract int DoPartTwo(T input);
}
