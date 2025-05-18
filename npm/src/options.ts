/** Options to configure the registry.
 *
 * In the browser, no tinkering should be needed.
 *
 * If you're using NextJS, you should consider adding the `nextjsServerSide` flag,
 * when on the server.
 */
export interface RegistryOptions {
  /** If set, this code is running on the server side of a NextJS app.
   *
   * This information is useful, because NextJS overrides the behavior of the
   * `fetch` API, which the registry uses to get an up-to-date remote registry.
   *
   * We want to cache these results, which `fetch` will do just fine in the browser.
   * But, by default, NextJS will not cache anything, ever.
   * By informing the registry that it's running in this environment, it can amend
   * its fetching behavior accordingly.
   */
  nextjsServerSide?: boolean;
}
