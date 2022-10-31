import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'
import { buildAccountsEndpoint } from '../endpoints/accounts'
import { buildCategoriesEndpoint } from '../endpoints/categories'
import { buildWalletEndpoint } from '../endpoints/wallets'
import { buildBondsEndpoint } from '../endpoints/bonds'

export type CancellableRequest<Params> = {
  signal: any,
  params: Params
}

export const api = createApi({
  reducerPath: 'api',
  tagTypes: ["Wallet", "Category", "Account", "Bond"],
  baseQuery: fetchBaseQuery({ baseUrl: '/api/' }),
  endpoints: (builder) => ({
    ...buildBondsEndpoint(builder),
    ...buildWalletEndpoint(builder),
    ...buildCategoriesEndpoint(builder),
    ...buildAccountsEndpoint(builder)
  }),
})

export const {
  useGetInterestRateHistoryQuery,
  useGetBondPerformanceQuery,
  useGetBondsBalanceQuery,
  useGetWalletsQuery,
  useCreateWalletMutation,
  useGetWalletQuery,
  useUpdateWalletMutation,
  useDestroyWalletMutation,
  useGetCategoriesQuery,
  useGetCategoryQuery,
  useLazyGetCategoryQuery,
  useLazyGetWalletQuery,
  useGetAccountQuery,
  useGetAccountsQuery,
  useLazyGetAccountQuery
} = api
