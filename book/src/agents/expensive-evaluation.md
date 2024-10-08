# Expensive Evaluation Functions

It's not unusual for your agent to have an evaluation function which is expensive to run. To mitigate this, we recommend using `RefCell` to store a cached value for each of your agents.
