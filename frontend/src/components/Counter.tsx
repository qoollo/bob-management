import React, { useState } from 'react';

function Counter() {
    // Initialize the count state to 0
    const [count, setCount] = useState(0);

    // Function to handle incrementing the count
    const increment = () => {
        setCount(count + 1);
    };

    // Function to handle decrementing the count
    const decrement = () => {
        setCount(count - 1);
    };

    return (
        <div>
            <h1>Counter</h1>
            <p>Count: {count}</p>
            <button onClick={increment}>Increment</button>
            <button onClick={decrement}>Decrement</button>
        </div>
    );
}

export default Counter;
