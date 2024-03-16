# TDD Bank Account

## TDD Manifesto Excercise 6

Note: This is an advanced example where the solution requires knowledge of using a mocking framework. The possible solution can also have an elaborated design. Solve it only if you feel comfortable with mocking frameworks and designing your code.

Create a simple bank application with features of depositing, withdrawing, and printing account statements.
Constraints

1. Start with a class with the following structure

public class Account {
  public void deposit(int amount)
  public void withdraw(int amount)
  public void printStatement()
}

2. You are not allowed to add any other public methods in this class

3. Use Strings and Integers for dates and amounts (keep it simple)

4. Don’t worry about the spacing in the statement printed in the console

### Requirements

1. Deposit into Account
2. Withdraw from an Account
3. Print the Account statement to the console

### Additional requirements added by me

1. Initial balance is 0.00
2. Each additional deposit increases the balance accordingly.
3. Any withdrawal that would decrease the balance below 0 is not allowed