/*
Módulo de Cálculos Percentuais e Problemas de Precificação

Este módulo fornece estruturas e métodos para resolver problemas matemáticos relacionados a preços e margens de lucro.

Exemplo de Problema: João vendeu sua bicicleta por R$280, o que representava 70% do valor
que ele havia pago inicialmente.
Pergunta: Por quanto ele deveria ter vendido a bicicleta para obter um lucro de 15%?
*/

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct PriceInfo {
    pub sale_price: Decimal,
    pub percentage_paid: Decimal,
}

impl PriceInfo {
    pub fn new(price: Decimal, percentage: Decimal) -> Self {
        assert!(percentage != dec!(0.0), "Percentage cannot be zero");
        Self {
            sale_price: price,
            percentage_paid: percentage,
        }
    }

    // Calcula o preço original baseado no preço de venda
    pub fn calculate_price(&self) -> Decimal {
        self.sale_price / self.percentage_paid
    }
}

pub struct ProfitCalculator {
    pub price_paid: PriceInfo,
    pub ideal_profit: Decimal,
}

impl ProfitCalculator {
    pub fn new(
        total_paid: Decimal,
        percentage_return: Decimal,
        sale_price_with_margin: Decimal,
    ) -> Self {
        assert!(
            sale_price_with_margin >= dec!(0.0),
            "Profit margin cannot be negative"
        );
        Self {
            price_paid: PriceInfo::new(total_paid, percentage_return),
            ideal_profit: sale_price_with_margin,
        }
    }

    // Calcula o preço de venda para atingir o lucro desejado
    pub fn compute_selling_price(&self) -> Decimal {
        let original_price = self.price_paid.calculate_price();
        original_price * (dec!(1.0) + self.ideal_profit)
    }
}

// Testes
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_price_calculation() {
        let price_to_find = PriceInfo::new(dec!(280.0), dec!(0.7));
        assert_eq!(price_to_find.calculate_price(), dec!(400.0))
    }
    #[test]
    fn test_selling_price_calculation() {
        let selling_with_return = ProfitCalculator::new(dec!(280.0), dec!(0.70), dec!(0.15));
        assert_eq!(selling_with_return.compute_selling_price(), dec!(460.00))
    }
}
