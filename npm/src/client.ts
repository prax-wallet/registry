import { BundledClient } from './bundled';
import { RemoteClient } from './remote';

export class ChainRegistryClient {
  public readonly bundled: BundledClient;
  public readonly remote: RemoteClient;

  constructor() {
    this.bundled = new BundledClient();
    this.remote = new RemoteClient(this.bundled);
  }
}
