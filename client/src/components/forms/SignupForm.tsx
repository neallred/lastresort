import { useState, FormEvent } from 'react';
import axios from 'axios';

const SignupForm = () => {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
        const apiRoot = 'http://localhost:8080'
        e.preventDefault();
        axios.post(`${apiRoot}/api/users`, {
            username,
            password,
        })
        //TO-DO: send user through login flow
        .then((response) => console.log(response))
        .catch((error) => console.log(error))
    };

    return (
        <div>
            <h3>Sign Up</h3>
            <form onSubmit={(e) => handleSubmit(e)}>
                <input type="text" name="username" value={username} onChange={(e) => setUsername(e.target.value)} />
                <input type="text" name="password" value={password} onChange={(e) => setPassword(e.target.value)} />
                <input type="submit" value="Submit"></input>
            </form>
        </div>
    )
}

export default SignupForm;
