import { createSlice } from '@reduxjs/toolkit';

export type User = {
  id: number,
  login: string
}

export type SessionStatus = 'initializing' | 'loading' | 'normal'

export interface ISessionState {
  user: User,
  status: SessionStatus
}

const initialState : ISessionState = {
  user: null,
  status: 'initializing'
}

export const sessionSlice = createSlice({
  name: 'session',
  initialState,
  reducers: {
    setStatus: (state, { payload } : { payload: SessionStatus }) => {
      state.status = payload
    },
    setCurrentUser(state, { payload } : { payload: User }) {
      state.user = payload
    }
  }
});

export const actions = sessionSlice.actions;
export default sessionSlice.reducer;
