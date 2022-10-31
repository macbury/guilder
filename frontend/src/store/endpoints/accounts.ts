import { GuilderEndpointBuilder } from ".";

export type Account = {
  currency: string,
  balance: string,
  description: string,
  name: string,
  id: number,
}

export const buildAccountsEndpoint = (builder : GuilderEndpointBuilder) => ({
  getAccounts: builder.query<Account[], void>({
    query: () => `accounts`,
    providesTags: ['Account'],
    transformResponse: (data : any) => data.accounts
  }),

  getAccount: builder.query<Account, number | string>({
    query: (id) => `accounts/${id}`,
    providesTags: ['Account'],
    transformResponse: (data : any) => data.account
  }),
})
