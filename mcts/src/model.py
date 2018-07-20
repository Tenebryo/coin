import tensorflow as tf

# with tf.name_scope('CoinNet') as scope:

#     input_size = 192
#     hidden_size = 256
#     prior_size = 64

#     #   The input to the neural network
#     net_input = tf.placeholder(tf.float32, [None, input_size], name='input')

#     #   Hidden layer 0
#     w0 = tf.Variable(tf.random_normal([input_size,hidden_size], stddev=0.005),name='weight_0')
#     b0 = tf.Variable(tf.random_normal([hidden_size], stddev=0.005), name='bias_0')

#     hl0 = tf.nn.dropout(tf.nn.relu(tf.matmul(net_input, w0) + b0),0.9)

#     #   Hidden layer 1
#     w1 = tf.Variable(tf.random_normal([hidden_size,hidden_size], stddev=0.005),name='weight_1')
#     b1 = tf.Variable(tf.random_normal([hidden_size], stddev=0.005), name='bias_1')

#     hl1 = tf.nn.dropout(tf.nn.relu(tf.matmul(hl0, w1) + b1),0.9)

#     #   Hidden layer 2
#     w2 = tf.Variable(tf.random_normal([2*hidden_size,hidden_size], stddev=0.005),name='weight_2')
#     b2 = tf.Variable(tf.random_normal([hidden_size], stddev=0.005), name='bias_2')

#     hl2 = tf.nn.dropout(tf.nn.relu(tf.matmul(tf.concat([hl1,hl0],1), w2) + b2),0.9)

#     #   Prior output layer
#     wp0 = tf.Variable(tf.random_normal([3*hidden_size,hidden_size], stddev=0.005),name='weight_p_0')
#     bp0 = tf.Variable(tf.random_normal([hidden_size], stddev=0.005), name='bias_p_0')

#     hlp = tf.nn.dropout(tf.nn.relu(tf.matmul(tf.concat([hl2,hl1,hl0],1), wp0) + bp0),0.9)

#     wp = tf.Variable(tf.random_normal([hidden_size,prior_size], stddev=0.005),name='weight_p')
#     bp = tf.Variable(tf.random_normal([prior_size], stddev=0.005), name='bias_p')

#     logits_p = tf.add(tf.matmul(hlp, wp), bp)
#     output_p = tf.nn.sigmoid(logits_p, name='output_p')

#     #   Value output layer
#     wv0 = tf.Variable(tf.random_normal([3*hidden_size,hidden_size], stddev=0.005),name='weight_v_0')
#     bv0 = tf.Variable(tf.random_normal([hidden_size], stddev=0.005), name='bias_v_0')

#     hlv = tf.nn.relu(tf.matmul(tf.concat([hl2,hl1,hl0],1), wv0) + bv0)

#     wv1 = tf.Variable(tf.random_normal([hidden_size,1], stddev=0.005),name='weight_v_1')
#     bv1 = tf.Variable(tf.random_normal([1], stddev=0.005), name='bias_v_1')

#     output_v = tf.tanh(tf.matmul(hlv, wv1) + bv1, name='output_v')

#     #   These are the supervized learning targets to train towards
#     net_target_p = tf.placeholder(tf.float32, [None, prior_size], name='target_p')
#     net_target_z = tf.placeholder(tf.float32, [None, 1], name='target_z')

#     #   This is the L2 regularization parameter
#     l2 = tf.placeholder(tf.float32, [], name='lambda')

#     # Weight the training by the whether the moves are actually important
#     n_shift = 1000
#     weights = (net_input[:,128:] + (1.0/(n_shift-1)))*((n_shift-1)/n_shift)
#     #   This is the regularized loss function
#     prior_loss = tf.reduce_mean(tf.losses.sigmoid_cross_entropy(net_target_p, logits_p, weights=weights))
#     value_loss = tf.reduce_mean(tf.squared_difference(net_target_z, output_v))
#     reg_loss = tf.contrib.layers.apply_regularization(tf.contrib.layers.l2_regularizer(scale=l2), tf.trainable_variables())

#     loss = tf.add(prior_loss + value_loss, reg_loss, name="loss")

#     #   This is the L2 regularization parameter
#     learning_rate = tf.placeholder(tf.float32, [], name='learning_rate')

#     optimizer_sgd = tf.train.GradientDescentOptimizer(learning_rate).minimize(loss, name='train_sgd')
#     optimizer_adm = tf.train.AdamOptimizer(learning_rate=learning_rate).minimize(loss, name='train_adm')
#     optimizer_mtn = tf.train.MomentumOptimizer(learning_rate=learning_rate, momentum=0.9, use_nesterov=True).minimize(loss, name='train_mtn')

#     init = tf.variables_initializer(tf.global_variables(), name = 'init')

#     saver = tf.train.Saver(tf.global_variables(), name='saver')

with tf.name_scope('CoinNet') as scope:

    input_size = 8
    input_features = 3
    real_input_size = input_size * input_size * input_features
    hidden_size = 128
    prior_size = 64

    conv_filters = [64,64,64,128,128]
    conv_kernels = [5,5,5,3,3]

    #   The input to the neural network
    real_net_input = tf.placeholder(tf.float32, [None, real_input_size], name='input')

    net_input = tf.reshape(real_net_input, [-1, input_size, input_size, input_features])

    conv = tf.layers.conv2d(
      inputs=net_input,
      filters=64,
      kernel_size=[5, 5],
      padding="same",
      activation=tf.nn.relu,
      name="conv0")

    conv = tf.layers.batch_normalization(
        inputs = conv,
        name = "conv_bn0")

    for i,(cs,ks) in enumerate(zip(conv_filters, conv_kernels)):
        if i == 0:
            continue

        conv = tf.layers.conv2d(
          inputs=conv,
          filters=cs,
          kernel_size=[ks, ks],
          padding="same",
          activation=tf.nn.relu,
          name="conv{}".format(i+1))

        conv = tf.layers.dropout(
            inputs = tf.layers.batch_normalization(
                inputs = conv,
                name = "conv_bn{}".format(i+1)),
            rate = 0.25,
            name = "conv_dropout{}".format(i+1))

    flat_conv = tf.reshape(conv, [-1, input_size * input_size * conv_filters[-1]])

    hidden_layer = tf.layers.dense(
        inputs = flat_conv, 
        units=hidden_size, 
        activation=tf.nn.relu,
        name='hidden_layer')

    hidden_p0 = tf.layers.dense(
        inputs = hidden_layer,
        units = hidden_size,
        activation = tf.nn.relu,
        name = 'hidden_p0')

    logits_p = tf.layers.dense(
        inputs = hidden_p0,
        units = prior_size,
        name = 'logits_p')

    output_p = tf.nn.softmax(logits_p, name = 'output_p')

    hidden_v = tf.layers.dense(
        inputs=hidden_layer,
        units = hidden_size,
        activation = tf.nn.relu,
        name = 'hidden_v')

    linear_v = tf.layers.dense(
        inputs = hidden_v,
        units = 1)

    output_v = tf.nn.tanh(linear_v, name = 'output_v')

    #   These are the supervized learning targets to train towards
    net_target_p = tf.placeholder(tf.float32, [None, prior_size], name='target_p')
    net_target_z = tf.placeholder(tf.float32, [None, 1], name='target_z')

    #   This is the L2 regularization parameter
    l2 = tf.placeholder(tf.float32, [], name='lambda')

    # Weight the training by the whether the moves are actually important
    n_shift = 1000
    weights = (real_net_input[:,128:] + (1.0/(n_shift-1)))*((n_shift-1)/n_shift)
    #   This is the regularized loss function
    # prior_loss = tf.reduce_mean(-tf.matmul(net_target_p, tf.log(output_p), transpose_a = True))
    prior_loss = tf.reduce_mean(tf.losses.softmax_cross_entropy(onehot_labels=net_target_p, logits=logits_p))
    # prior_loss = tf.reduce_mean(tf.losses.sigmoid_cross_entropy(net_target_p, logits_p, weights=weights))
    value_loss = tf.reduce_mean(tf.squared_difference(net_target_z, output_v))
    reg_loss = tf.contrib.layers.apply_regularization(tf.contrib.layers.l2_regularizer(scale=l2), tf.trainable_variables())

    loss = tf.add(prior_loss + value_loss, reg_loss, name="loss")

    #   This is the L2 regularization parameter
    learning_rate = tf.placeholder(tf.float32, [], name='learning_rate')

    optimizer_sgd = tf.train.GradientDescentOptimizer(learning_rate).minimize(loss, name='train_sgd')
    optimizer_adm = tf.train.AdamOptimizer(learning_rate=learning_rate).minimize(loss, name='train_adm')
    optimizer_mtn = tf.train.MomentumOptimizer(learning_rate=learning_rate, momentum=0.9, use_nesterov=True).minimize(loss, name='train_mtn')

    init = tf.variables_initializer(tf.global_variables(), name = 'init')

    saver = tf.train.Saver(tf.global_variables(), name='saver')
saver_def = saver.as_saver_def()

print ('Saver Information:')

# The name of the tensor you must feed with a filename when saving/restoring.
print ('  Filename: {}'.format(saver_def.filename_tensor_name))

# The name of the target operation you must run when restoring.
print ('  Restore: {}'.format(saver_def.restore_op_name))

# The name of the target operation you must run when saving.
print ('  Save: {}'.format(saver_def.save_tensor_name))

total_params = 0
for i in tf.trainable_variables():
    local_params = 1
    for j in i.get_shape():
        local_params *= int(j)
    total_params += local_params

print ('Total Model Parameters: {}'.format(total_params))

#   Save the 
definition = tf.Session().graph_def
directory = './data/'
tf.train.write_graph(definition, directory, 'CoinNet_model.pb', as_text=False)

# import random

# sess = tf.Session()
# sess.run([init])

# print('Loading Data...')
# file_path = './data/prior_test_data.txt'
# data = []
# data_in = []
# data_out = []
# with open(file_path, 'r') as f:
#     for ln in f.readlines():
#         s = ln.split(' ')
#         try:
#             data.append( ([float(x) for x in s[0]][:192], [float(x) for x in s[1].strip()][:64]) )
#         except Exception,e:
#             print (s)
#             print (e)
#         if random.random() > 2:
#             break
#     print('Shuffling Data...')
#     random.shuffle(data)

#     data_in = [x[0] for x in data]
#     data_out = [x[1] for x in data]

# print('Done Loading Data!')

# fd = {}
# def do_epoch():
#     global fd
#     batch_size = 128 
#     total_batch = len(data)/batch_size
#     for i in range(total_batch):
#         ix = i * batch_size
#         batch_xs = data_in[ix:ix+batch_size]
#         batch_ys = data_out[ix:ix+batch_size]

#         fd = {}
#         fd[net_input] = batch_xs
#         fd[net_target_p] = batch_ys
#         fd[net_target_z] = [[0.5]]
#         fd[learning_rate] = 0.0001
#         fd[l2] = 0.0001
#         # Run optimization op (backprop) and cost op (to get loss value)
#         l,_ = sess.run([loss, optimizer_mtn],feed_dict=fd)
#         print "Loss: {}".format(l)

# fd[net_input] = [data_in[0]]
# fd[net_target_p] = [data_out[0]]
# fd[net_target_z] = [[0.5]]
# fd[learning_rate] = 0.0001
# fd[l2] = 0.0001