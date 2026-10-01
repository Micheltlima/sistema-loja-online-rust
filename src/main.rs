# SISTEMA DE VENDAS ONLINE

# Estoque inicial
produtos = {
    1: {"nome": "Notebook", "preco": 3500.00, "estoque": 10},
    2: {"nome": "Smartphone", "preco": 2200.00, "estoque": 15},
    3: {"nome": "Teclado", "preco": 150.00, "estoque": 20},
    4: {"nome": "Mouse", "preco": 80.00, "estoque": 25},
    5: {"nome": "Monitor", "preco": 900.00, "estoque": 8},
    6: {"nome": "Impressora", "preco": 750.00, "estoque": 7},
    7: {"nome": "Headset", "preco": 180.00, "estoque": 18},
    8: {"nome": "Webcam", "preco": 250.00, "estoque": 12},
    9: {"nome": "Caixa de Som", "preco": 300.00, "estoque": 10},
    10: {"nome": "HD Externo", "preco": 450.00, "estoque": 9},
    11: {"nome": "Pen Drive", "preco": 60.00, "estoque": 30},
    12: {"nome": "Cadeira Gamer", "preco": 1200.00, "estoque": 6},
}

# Lista para armazenar os produtos comprados
carrinho = []


def exibir_produtos():
    print("\n========== PRODUTOS DISPONÍVEIS ==========")

    for codigo, produto in produtos.items():
        print(
            f"Código: {codigo:2} | "
            f"Produto: {produto['nome']:<18} | "
            f"Preço: R$ {produto['preco']:8.2f} | "
            f"Estoque: {produto['estoque']}"
        )

    print("===========================================")


def comprar_produto():
    exibir_produtos()

    # Validação do código do produto
    while True:
        try:
            codigo = int(input("\nDigite o código do produto: "))

            if codigo not in produtos:
                print("Erro: produto não encontrado. Digite um código válido.")
                continue

            break

        except ValueError:
            print("Erro: digite apenas números.")

    produto = produtos[codigo]

    # Validação da quantidade
    while True:
        try:
            quantidade = int(
                input(f"Digite a quantidade de {produto['nome']}: ")
            )

            if quantidade <= 0:
                print("Erro: a quantidade deve ser maior que zero.")
                continue

            if quantidade > produto["estoque"]:
                print(
                    f"Erro: estoque insuficiente. "
                    f"Disponível: {produto['estoque']}"
                )
                continue

            break

        except ValueError:
            print("Erro: digite uma quantidade inteira válida.")

    # Atualiza o estoque
    produto["estoque"] -= quantidade

    valor_item = produto["preco"] * quantidade

    # Verifica se o produto já está no carrinho
    encontrado = False

    for item in carrinho:
        if item["codigo"] == codigo:
            item["quantidade"] += quantidade
            item["valor"] += valor_item
            encontrado = True
            break

    # Caso ainda não esteja no carrinho
    if not encontrado:
        carrinho.append({
            "codigo": codigo,
            "nome": produto["nome"],
            "preco": produto["preco"],
            "quantidade": quantidade,
            "valor": valor_item
        })

    print("\nProduto adicionado à compra com sucesso!")


def exibir_relatorio():
    print("\n")
    print("=" * 70)
    print("                    RELATÓRIO DE VENDA")
    print("=" * 70)

    if len(carrinho) == 0:
        print("Nenhum produto foi comprado.")
    else:
        print(f"{'Produto':<20} {'Qtd.':>6} {'Preço Unit.':>15} {'Valor':>15}")
        print("-" * 70)

        total = 0

        for item in carrinho:
            print(
                f"{item['nome']:<20} "
                f"{item['quantidade']:>6} "
                f"R$ {item['preco']:>11.2f} "
                f"R$ {item['valor']:>11.2f}"
            )

            total += item["valor"]

        print("-" * 70)
        print(f"{'VALOR TOTAL DA COMPRA:':<50} R$ {total:>11.2f}")

    # Identificadores solicitados no enunciado
    print("\nIdentificadores:")
    print("Piano")
    print("Kit Churrasco")
    print("Deck Box")

    print("=" * 70)
    print("Obrigado pela compra!")
    print("=" * 70)


# Programa principal
while True:
    print("\n========== SISTEMA DE VENDAS ==========")
    print("1 - Consultar produtos")
    print("2 - Comprar produto")
    print("3 - Finalizar compra")
    print("========================================")

    opcao = input("Escolha uma opção: ")

    if opcao == "1":
        exibir_produtos()

    elif opcao == "2":
        comprar_produto()

    elif opcao == "3":
        exibir_relatorio()
        break

    else:
        print("Erro: opção inválida. Escolha 1, 2 ou 3.")
