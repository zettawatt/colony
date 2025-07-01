let password: string | null = null;

export function setPassword(pw: string) {
  password = pw;
}

export function getPassword(): string | null {
  return password;
}

export function clearPassword() {
  password = null;
}
