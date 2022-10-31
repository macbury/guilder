import { createSlice, createEntityAdapter } from '@reduxjs/toolkit';
import { fetchAll, fetch, Requests } from '../api/bonds';
import { showError } from '../../toasts';

type Errors = { [key:string]: string[] }

export type Bond = {
  endDate: string;
  name: string,
  id: number,
  uid: string,
  status: 'Active' | 'Archived',
  kind: string,
  categoryId: number,
  walletId: number,
  integrationId: number,
  accountId: number,
  startDate: string,
  interestDate: string,
  updatedAt: string,
  currency: string,
  emission: string,
  performance: {
    shares: number,
    buyoutDaysLeft: number,
    interestDaysLeft: number,
    lastPrice: number,
    priceChange: number,
    percentChange: number,
    dayPercentChange: number,
    startPrice: number,
    price: number,
    currentRate: number,
    buyoutPrice: number,
    dayPriceChange: number,
    rates: number[]
  }
}

export type BondFormValue = {
  categoryId: number,
  updateCategory?: boolean,
  accountId: number,
  updateAccount?: boolean,
  walletId: number,
  updateWallet?: boolean
}

export type BondMassEditForm = {
  visible: boolean,
  value: BondFormValue,
  errors: Errors,
};

type Bonds = Bond[]

export interface IBondsState {
  loading: boolean,
  form: BondMassEditForm,
  currentBondId: number,
  selectedIds: {
    [id: number]: boolean
  }
}

export const bondsAdapter = createEntityAdapter<Bond>();

const initialState = bondsAdapter.getInitialState<IBondsState>({
  loading: true,
  currentBondId: null,
  selectedIds: {},
  form: { value: { categoryId: null, accountId: null, walletId: null }, visible: false, errors: {} }
});

export const bondsSlice = createSlice({
  name: 'bonds',
  initialState,
  reducers: {
    resetForm: (state) => {
      state.form.value = {
        categoryId: null,
        accountId: null,
        walletId: null,
        updateCategory: false,
        updateAccount: false,
        updateWallet: false,
      };
    },

    updateForm: (state, { payload } : { payload: BondFormValue }) => {
      state.form.value = payload
    },

    clearErrors: (state) => {
      state.form.errors = {};
    },

    setErrors: (state, { payload } : { payload: Errors }) => {
      state.form.errors = payload;
    },

    showForm: (state) => {
      state.form.visible = true;
    },

    hideForm: (state) => {
      state.form.visible = false;
    },

    setLoading: (state, { payload } : { payload: boolean }) => {
      state.loading = payload
    },

    selectAll: (state) => {
      state.selectedIds = state.ids.reduce((map, id) => {
        map[id] = true;
        return map
      }, {});
    },

    clearSelection: (state) => {
      state.selectedIds = {};
    },

    deselect: (state, { payload } : { payload: number }) => {
      state.selectedIds[payload] = false
    },

    select: (state, { payload } : { payload: number }) => {
      state.selectedIds[payload] = true
    },

    setBonds: (state, { payload } : { payload: Bonds }) => {
      bondsAdapter.setAll(state, payload)
    },

    setCurrentBondId: (state, { payload } : { payload: number }) => {
      state.currentBondId = payload
    },

    setBond: (state, { payload } : { payload: Bond }) => {
      bondsAdapter.setOne(state, payload)
    },

    updateBond: (state, { payload } : { payload: Bond }) => {
      bondsAdapter.updateOne(state, {
        id: payload.id,
        changes: payload
      })
    },

    removeBond: (state, { payload } : { payload: number }) => {
      bondsAdapter.removeOne(state, payload);
    }
  },
  extraReducers: builder => (
    builder
      .addCase(fetchAll.fulfilled, (state, { payload }) => {
        bondsAdapter.setAll(state, payload)
        state.loading = false
      })
      .addCase(fetch.fulfilled, (state, { payload }) => {
        bondsAdapter.upsertOne(state, payload)
        state.loading = false
      })
      .addMatcher(Requests.started, (state) => {
        state.loading = true
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

export const actions = bondsSlice.actions;
export default bondsSlice.reducer;
