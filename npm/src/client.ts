import { BundledClient } from './bundled';
import { RegistryOptions } from './options';
import { RemoteClient } from './remote';

export class ChainRegistryClient {
  public readonly bundled: BundledClient;
  public readonly remote: RemoteClient;

  constructor(options?: RegistryOptions) {
    this.bundled = new BundledClient();
    this.remote = new RemoteClient(this.bundled, options);
  }
}
