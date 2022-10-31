import { createAsyncThunk, isAnyOf } from '@reduxjs/toolkit';
import axios from 'axios';
import { CancellableRequest } from '.';
import { FilterOptions, mapSortKey } from '../hooks/filters';
import { Asset } from '../slices/assets';

export type AssetsFilterOptions = FilterOptions & {

}

export const fetchAll = createAsyncThunk('assets/all', async ({ params, signal } : CancellableRequest<AssetsFilterOptions>) => {
  const mappedParams = {
    ...params,
    direction: params.direction,
    sort: mapSortKey(params.sort)
  }
  const { data: { assets } } = await axios.get("/api/assets", { params: mappedParams, signal });

  return assets
})

export const destroy = createAsyncThunk('assets/destroy', async (ticker : string) => {
  await axios.delete(`/api/assets/${ticker}`);
  return ticker
})

export const fetch = createAsyncThunk('assets/fetch', async (ticker : string) => {
  const { data } = await axios.get(`/api/assets/${ticker}`);
  return data.asset as Asset
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
