import { useState, FormEvent } from 'react';
import axios from 'axios';

const CreateUser = () => {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        axios.post('localhost:8080/api/users', {
            username,
            password,
        })
        //TO-DO: send user somewhere, set a cookie
        .then((response) => console.log(response))
        .catch((error) => console.log(error))
    };

    return (
        <div>
            <h3>Create a User Form</h3>
            <form onSubmit={(e) => handleSubmit(e)}>
                <input type="text" name="username" value={username} onChange={(e) => setUsername(e.target.value)} />
                <input type="text" name="password" value={password} onChange={(e) => setPassword(e.target.value)} />
                <input type="submit" value="Submit"></input>
            </form>
        </div>
    )
}

export default CreateUser;