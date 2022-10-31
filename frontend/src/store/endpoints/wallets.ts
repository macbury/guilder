import { GuilderEndpointBuilder } from ".";

export type Wallet = {
  id: number,
  balance: number,
  name: string,
  currency: string,
  description: string
}

export const buildWalletEndpoint = (builder : GuilderEndpointBuilder) => ({
  getWallets: builder.query<Wallet[], void>({
    query: () => `wallets`,
    providesTags: ['Wallet'],
    transformResponse: (data : any) => data.wallets
  }),
  getWallet: builder.query<Wallet, number | string>({
    query: (id) => `wallets/${id}`,
    providesTags: ['Wallet'],
    transformResponse: (data : any) => data.wallet
  }),
  createWallet: builder.mutation<{ success: boolean, resource: Wallet, errors: any }, Omit<Wallet, 'id'>>({
    query: (wallet) => ({
      url: 'wallets',
      method: 'POST',
      body: wallet,
    }),
    invalidatesTags: ['Wallet']
  }),
  updateWallet: builder.mutation<{ success: boolean, resource: Wallet, errors: any }, { wallet: Omit<Wallet, 'id'>, id: number | string }>({
    query: ({ id, wallet }) => ({
      url: `wallets/${id}`,
      method: 'PUT',
      body: wallet,
    }),
    invalidatesTags: ['Wallet']
  }),
  destroyWallet: builder.mutation<void, number | string>({
    query: (id) => ({
      url: `wallets/${id}`,
      method: 'DELETE',
    }),
    invalidatesTags: ['Wallet']
  }),
})
