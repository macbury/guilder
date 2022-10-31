import { createSlice, createEntityAdapter } from '@reduxjs/toolkit';
import { showError } from '../../toasts';
import { fetchAll, destroy, fetch, Requests } from '../api/assets';

type Errors = { [key:string]: string[] }
type Ticker = string;

export type AssetPerformance = {
  price: number,
  lastPrice: number,
  priceChange: number,
  percentChange: number,
  yearlyChange: number,
  yearlyPercentChange: number,
  highValue: number,
  lowValue: number,
  lowHighScore: number,
  ytdPrice: number,
  ytdChange: number,
  ytdPercentChange: number
}

export type Asset = {
  id: Ticker,
  name: string,
  description: string,
  websiteUrl: string,
  currency: string,
  country: string,
  exchange: string,
  isin: string,
  logoUrl: string,
  secondaryLogoUrl: string,
  performance: AssetPerformance
}

export type Assets = Asset[];

export type AssetFormValue = {
  categoryId: number
}

export type AssetMassEditForm = {
  visible: boolean,
  value: AssetFormValue,
  errors: Errors,
};

export interface IAssetsState {
  loading: boolean,
  currentAssetId: string,
  form: AssetMassEditForm,
  selectedIds: {
    [ticker: Ticker]: boolean
  },
}

export const assetsAdapter = createEntityAdapter<Asset>();

const initialState = assetsAdapter.getInitialState<IAssetsState>({
  loading: true,
  currentAssetId: null,
  selectedIds: {},
  form: { value: { categoryId: null }, visible: false, errors: {} }
});

export const assetsSlice = createSlice({
  name: 'assets',
  initialState,
  reducers: {
    resetForm: (state) => {
      state.form.value = {
        categoryId: null
      };
    },

    updateForm: (state, { payload } : { payload: AssetFormValue }) => {
      state.form.value = payload
    },

    clearErrors: (state) => {
      state.form.errors = {};
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

    setErrors: (state, { payload } : { payload: Errors }) => {
      state.form.errors = payload;
    },

    showForm: (state) => {
      state.form.visible = true;
    },

    hideForm: (state) => {
      state.form.visible = false;
    },

    deselect: (state, { payload } : { payload: Ticker }) => {
      state.selectedIds[payload] = false
    },

    select: (state, { payload } : { payload: Ticker }) => {
      state.selectedIds[payload] = true
    },

    setLoading: (state, { payload } : { payload: boolean }) => {
      state.loading = payload
    },

    setCurrentAssetId: (state, { payload } : { payload: Ticker }) => {
      state.currentAssetId = payload
    },

    updateAssets: (state, { payload } : { payload: Assets }) => {
      const currentAsset = payload.find(({ id }) => id == state.currentAssetId);
      if (currentAsset) {
        assetsAdapter.upsertOne(state, currentAsset);
      }

      const updates = payload.map((item) => ({
        id: item.id,
        changes: item
      }));
      assetsAdapter.updateMany(state, updates);
    },

    setAsset: (state, { payload } : { payload: Asset }) => {
      assetsAdapter.setOne(state, payload)
    },

    removeAsset: (state, { payload } : { payload: string }) => {
      assetsAdapter.removeOne(state, payload);
    }
  },
  extraReducers: builder =>
    builder
      .addCase(destroy.fulfilled, (state, { payload: id }) => {
        assetsAdapter.removeOne(state, id)
      })
      .addCase(fetchAll.fulfilled, (state, action) => {
        assetsAdapter.setAll(state, action.payload)
      })
      .addCase(fetch.fulfilled, (state, { payload }) => {
        if (payload) {
          assetsAdapter.upsertOne(state, payload);
          state.currentAssetId = payload.id
        }
      })
      .addMatcher(Requests.started, (state) => {
        state.loading = true
      })
      .addMatcher(Requests.finished, (state) => {
        state.loading = false
      })
      .addMatcher(Requests.failed, (state, { error }) => {
        if (error.name !== "AbortError") {
          console.error("Aborted request", error);
        } else {
          showError(error.toString());
        }
      })
});

export const actions = assetsSlice.actions;
export default assetsSlice.reducer;
