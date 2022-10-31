import { createSlice, createEntityAdapter } from '@reduxjs/toolkit';


type Errors = { [key:string]: string[] }

export type AccountFormValue = {
  name: string,
  description: string,
  currency: string
}

export interface IAccountsState {
  loading: boolean,
  form: {
    visible: boolean,
    value: AccountFormValue,
    accountId: number,
    errors: Errors,
  }
}

const initialState = {
  loading: true,
  form: {
    visible: false,
    errors: {},
    accountId: null,
    value: { name: '', description: '', currency: '' }
  }
}

export const accountsSlice = createSlice({
  name: 'accounts',
  initialState,
  reducers: {
    setLoading: (state, { payload } : { payload: boolean }) => {
      state.loading = payload
    },

    setAccountId: (state, { payload } : { payload: number }) => {
      state.form.accountId = payload
    },

    setErrors: (state, { payload } : { payload: Errors }) => {
      state.form.errors = payload
    },

    resetForm: (state) => {
      state.form.errors = {};
      state.form.accountId = null;
      state.form.value.name = '';
      state.form.value.description = '';
      state.form.value.currency = '';
    },

    openForm: (state) => {
      state.form.visible = true;
    },

    updateForm: (state, { payload } : { payload: AccountFormValue }) => {
      state.form.value = payload;
    },

    closeForm: (state) => {
      state.form.visible = false;
    }
  }
});

export const actions = accountsSlice.actions;
export default accountsSlice.reducer;
