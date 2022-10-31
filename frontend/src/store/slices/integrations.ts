import { createSlice, createEntityAdapter, createSelector } from '@reduxjs/toolkit';

export type Integration = {
  name: string,
  id: number,
}

type Integrations = Integration[]
type Errors = { [key:string]: string[] }

export type IntegrationFormValue = {
  name: string,
  login: string,
  password: string,
  kind: string
}

export interface IIntegrationsState {
  loading: boolean,
  form: {
    visible: boolean,
    value: IntegrationFormValue,
    integrationId: number,
    errors: Errors,
  }
}

export const integrationsAdapter = createEntityAdapter<Integration>();

const initialState = integrationsAdapter.getInitialState<IIntegrationsState>({
  loading: true,
  form: {
    visible: false,
    errors: {},
    integrationId: null,
    value: { name: '', login: '', password: '', kind: '' }
  }
});

export const integrationsSlice = createSlice({
  name: 'integrations',
  initialState,
  reducers: {
    setLoading: (state, { payload } : { payload: boolean }) => {
      state.loading = payload
    },

    setIntegrations: (state, { payload } : { payload: Integrations }) => {
      integrationsAdapter.setAll(state, payload)
    },

    setIntegration: (state, { payload } : { payload: Integration }) => {
      integrationsAdapter.setOne(state, payload)
    },

    setIntegrationId: (state, { payload } : { payload: number }) => {
      state.form.integrationId = payload
    },

    removeIntegration: (state, { payload } : { payload: number }) => {
      integrationsAdapter.removeOne(state, payload);
    },

    setErrors: (state, { payload } : { payload: Errors }) => {
      state.form.errors = payload
    },

    resetForm: (state) => {
      state.form.errors = {};
      state.form.integrationId = null;
      state.form.value.name = '';
      state.form.value.login = '';
      state.form.value.password = '';
      state.form.value.kind = '';
    },

    openForm: (state) => {
      state.form.visible = true;
    },

    updateForm: (state, { payload } : { payload: IntegrationFormValue }) => {
      state.form.value = payload;
    },

    closeForm: (state) => {
      state.form.visible = false;
    }
  }
});

export const actions = integrationsSlice.actions;
export default integrationsSlice.reducer;
