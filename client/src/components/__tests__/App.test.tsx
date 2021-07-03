import { render, screen } from '@testing-library/react';
import App from '../App';

test('renders last resort homepage with login and signup forms', () => {
  render(<App />);
  const header = screen.getByText("Last Resort");
  expect(header).toBeInTheDocument();

  const loginForm = screen.getByText("Log in");
  expect(loginForm).toBeInTheDocument();

  const signupForm = screen.getByText("Sign Up");
  expect(signupForm).toBeInTheDocument();
});
