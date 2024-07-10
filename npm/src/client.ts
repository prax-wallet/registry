import { BundledClient } from './bundled';
import { RemoteClient } from './remote';

export class ChainRegistryClient {
  public readonly bundled = new BundledClient();
  public readonly remote = new RemoteClient();
}
