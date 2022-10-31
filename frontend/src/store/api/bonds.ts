import { createAsyncThunk, isAnyOf } from '@reduxjs/toolkit';
import axios from 'axios';
import { CancellableRequest } from '.';
import { FilterOptions, mapSortKey } from '../hooks/filters';

export type BondFilterOptions = FilterOptions & {
  accounts: number[],
  wallets: number[],
  scope: 'all' | 'active' | 'archived'
}

export const fetchAll = createAsyncThunk('bonds/all', async ({ params, signal } : CancellableRequest<BondFilterOptions>) => {
  const mappedParams = {
    ...params,
    direction: params.direction,
    sort: mapSortKey(params.sort)
  }
  const { data: { bonds } } = await axios.get("/api/bonds", { params: mappedParams, signal });

  return bonds
})

export const fetch = createAsyncThunk('bonds/fetch', async ({ params: { bondId }, signal } : CancellableRequest<{ bondId: number }>) => {
  const { data: { bond } } = await axios.get("/api/bonds/"+bondId, { params: signal });

  return bond
})

export const destroy = createAsyncThunk('bonds/destroy', async (bondId : number) => {
  await axios.delete(`/api/bonds/${bondId}`);
  return { bondId }
})

export const Requests = {
  failed: isAnyOf(
    fetch.rejected,
    fetchAll.rejected,
    destroy.rejected
  ),
  finished: isAnyOf(
    fetch.fulfilled,
    fetchAll.fulfilled,
    destroy.fulfilled
  ),
  started: isAnyOf(
    fetch.pending,
    fetchAll.pending,
    destroy.pending
  )
}
