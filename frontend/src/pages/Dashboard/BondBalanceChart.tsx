import React from 'react';
import { MdAccountBalance, MdOutlineAreaChart, MdOutlineScore, MdOutlineDirections, MdOutlineStairs } from "react-icons/md";
import { usePriceFormatter } from '../../components/hooks/useFormattedPrice';
import { useGetBondsBalanceQuery } from '../../store/api';
import SimpleHistoryChart from '../../components/Charts/SimpleHistoryChart';

export function BondMonthlyInterest() {
  const { data, isLoading } = useGetBondsBalanceQuery();
  const formatPrice = usePriceFormatter("PLN")

  return (
    <SimpleHistoryChart
      icon={<MdAccountBalance />}
      title="Bond monthly interests"
      data={data?.performance}
      primaryValueKey="monthPriceChange"
      dateValueKey="month"
      loading={isLoading}
      color="#8ef26d"
      format={formatPrice}
    />
  )
}

export function BondMonthlyBalance() {
  const { data, isLoading } = useGetBondsBalanceQuery();
  const formatPrice = usePriceFormatter("PLN")

  return (
    <SimpleHistoryChart
      icon={<MdOutlineAreaChart />}
      title="Bond total interests"
      data={data?.performance}
      primaryValueKey="totalPriceChange"
      dateValueKey="month"
      loading={isLoading}
      color="#3949AB"
      format={formatPrice}
    />
  )
}

export function BondBuyoutPrice() {
  const { data, isLoading } = useGetBondsBalanceQuery();
  const formatPrice = usePriceFormatter("PLN")

  return (
    <SimpleHistoryChart
      icon={<MdOutlineScore />}
      title="Bonds total buyout interests"
      data={data?.performance}
      primaryValueKey="totalBuyoutPriceChange"
      dateValueKey="month"
      loading={isLoading}
      color="#fcf800"
      format={formatPrice}
    />
  )
}


export function BondBuyoutMonthlyPrice() {
  const { data, isLoading } = useGetBondsBalanceQuery();
  const formatPrice = usePriceFormatter("PLN")

  return (
    <SimpleHistoryChart
      icon={<MdOutlineDirections />}
      title="Bonds month buyout interests"
      data={data?.performance}
      primaryValueKey="monthTotalBuyoutChange"
      dateValueKey="month"
      loading={isLoading}
      color="#3CF"
      format={formatPrice}
    />
  )
}

export function BondBalancePrice() {
  const { data, isLoading } = useGetBondsBalanceQuery();
  const formatPrice = usePriceFormatter("PLN")

  return (
    <SimpleHistoryChart
      icon={<MdOutlineStairs />}
      title="Bonds total value"
      data={data?.performance}
      primaryValueKey="price"
      dateValueKey="month"
      loading={isLoading}
      color="#FF9000"
      format={formatPrice}
    />
  )
}
