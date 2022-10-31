import { GuilderEndpointBuilder } from ".";


export type InterestRateHistory = {
  bondId: number,
  currency: string,
  date: string,
  startPrice: number,
  period: number,
  price: number,
  rate: number,
  lastPrice: number,
  priceChange: number,
  percentChange: number,
  dayPriceChange: number,
  dayPercentChange: number
}

export type BondMonthlyPerformance = {
  bondId: number,
  date: string,
  currency: string,
  period: number,
  price: number,
  priceChange: number,
  percentChange: number
}

export type BondPeriod = {
  index: number,
  rate: number,
  startDate: string,
  endDate: string
}

export type BondInterestRateHistory = {
  periods: BondPeriod[]
  history: InterestRateHistory[]
}

export type BondBalance = {
  currency: string,
  month: string,
  monthPercentChange: number,
  monthPriceChange: number,
  price: number,
  startPrice: number,
  totalPercentChange: number,
  totalPriceChange: number
}


export const buildBondsEndpoint = (builder : GuilderEndpointBuilder) => ({
  getInterestRateHistory: builder.query<BondInterestRateHistory, number>({
    query: (id) => `bonds/${id}/interest_rates`,
    providesTags: ['Bond'],
  }),
  getBondPerformance: builder.query<{ performance: BondMonthlyPerformance[] }, number>({
    query: (id) => `bonds/${id}/performance`,
    providesTags: ['Bond'],
  }),
  getBondsBalance: builder.query<{ performance: BondBalance[] }, void>({
    query: () => `bonds/balances`,
    providesTags: ['Bond'],
  }),
})
