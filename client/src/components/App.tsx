import LoginForm from './forms/LoginForm'
import SignupForm from './forms/SignupForm'

const App = () => {
  return (
    <div className="App">
      <h1>Last Resort</h1>
      <LoginForm />
      <p>Don't have an account yet? Sign up!</p>
      <SignupForm />
      <p>Or browse postings!</p>

    </div>
  );
}

export default App;
