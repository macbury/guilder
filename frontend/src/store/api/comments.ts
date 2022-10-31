import { createAsyncThunk, isAnyOf } from '@reduxjs/toolkit';
import axios from 'axios';
import { CancellableRequest } from '.';

export type ModelType = 'Asset' | 'Bond';
export type FetchCommentsParams = {
  modelId : string,
  modelType: ModelType
}

export const fetchAll = createAsyncThunk('comments/all', async ({ params: { modelId, modelType }, signal } : CancellableRequest<FetchCommentsParams>) => {
  const { data: { comments } } = await axios.get(`/api/comments/${modelType}/${modelId}`, { signal });
  return comments
})

export const destroy = createAsyncThunk('comments/destroy', async (commentId : number) => {
  await axios.delete(`/api/comments/${commentId}`);
  return { commentId }
})

export const Requests = {
  failed: isAnyOf(
    destroy.rejected,
    fetchAll.rejected,
  ),
  finished: isAnyOf(
    destroy.fulfilled,
    fetchAll.fulfilled,
  ),
  started: isAnyOf(
    destroy.pending,
    fetchAll.pending,
  )
}
