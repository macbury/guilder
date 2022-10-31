import { createSlice, createEntityAdapter, createSelector } from '@reduxjs/toolkit';
import { destroy, fetchAll, Requests } from '../api/comments';
import { showError } from '../../toasts';

export type Comment = {
  body: string,
  id: number,
  date: Date
}

type Comments = Comment[]
type Errors = { [key:string]: string[] }

export interface ICommentsState {
  loading: boolean,
  errors: Errors,
  body: string,
  modelId : string | number,
  modelType: string
}

export const commentsAdapter = createEntityAdapter<Comment>();

const initialState = commentsAdapter.getInitialState<ICommentsState>({
  loading: true,
  errors: {},
  body: '',
  modelId: null,
  modelType: null
});

export const commentsSlice = createSlice({
  name: 'comments',
  initialState,
  reducers: {
    setModel: (state, { payload: { modelId, modelType } } : { payload: { modelId : string, modelType: string } }) => {
      state.modelId = modelId;
      state.modelType = modelType;
    },

    setLoading: (state, { payload } : { payload: boolean }) => {
      state.loading = payload
    },

    setBody: (state, { payload } : { payload: string }) => {
      state.body = payload
    },

    setErrors: (state, { payload } : { payload: Errors }) => {
      state.errors = payload
    },

    setComments: (state, { payload } : { payload: Comments }) => {
      commentsAdapter.setAll(state, payload)
    },

    addComment: (state, { payload } : { payload: Comment }) => {
      commentsAdapter.addOne(state, payload)
    },

    removeComment: (state, { payload } : { payload: number }) => {
      commentsAdapter.removeOne(state, payload);
    }
  },
  extraReducers: builder => (
    builder
      .addCase(destroy.fulfilled, (state, { payload: { commentId } }) => {
        commentsAdapter.removeOne(state, commentId)
      })
      .addCase(fetchAll.fulfilled, (state, { payload }) => {
        commentsAdapter.setAll(state, payload)
        state.loading = false
      })
      .addMatcher(Requests.started, (state) => {
        state.loading = true
      })
      .addMatcher(Requests.finished, (state) => {
        state.loading = false
      })
      .addMatcher(Requests.failed, (state, { error }) => {
        console.log('Handled?')
        if (error.name !== "AbortError") {
          console.error("Aborted fetching", error);
        } else {
          showError(error.toString());
        }
        state.loading = false
      })
  )
});

export const actions = commentsSlice.actions;
export default commentsSlice.reducer;
