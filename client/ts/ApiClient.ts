class ApiClient {
  public constructor(private endpoint: string) {}
}

const client = new ApiClient(`http://localhost:3000`);
