import tensorflow as tf

with tf.name_scope('CoinNet') as scope:

    input_size = 8
    input_features = 3
    real_input_size = input_size * input_size * input_features
    hidden_size = 128
    prior_size = 64

    conv_filters = [128,64,64,128]
    conv_kernels = [5,5,3,3]

    #   The input to the neural network
    real_net_input = tf.placeholder(tf.float32, [None, real_input_size], name='input')

    net_input = tf.reshape(real_net_input, [-1, input_size, input_size, input_features])

    conv = tf.layers.conv2d(
      inputs=net_input,
      filters=conv_filters[0],
      kernel_size=[conv_kernels[0], conv_kernels[0]],
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

        # conv = tf.layers.dropout(
        #     inputs = tf.layers.batch_normalization(
        #         inputs = conv,
        #         name = "conv_bn{}".format(i+1)),
        #     rate = 0.25,
        #     name = "conv_dropout{}".format(i+1))

        conv = tf.layers.batch_normalization(
                inputs = conv,
                name = "conv_bn{}".format(i+1))

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

    # the softmax is masked by the valid mobility of the current player so that
    # invalid moves don't affect the softmax and thus don't learn something
    # they shouldn't. This is the policy output of the network
    output_p = tf.nn.softmax(
        logits_p * tf.reshape(net_input[:,:,:,2], [-1,input_size * input_size]), 
        name = 'output_p')

    hidden_v = tf.layers.dense(
        inputs=hidden_layer,
        units = hidden_size,
        activation = tf.nn.relu,
        name = 'hidden_v')

    linear_v = tf.layers.dense(
        inputs = hidden_v,
        units = 1)

    # This is the value output of the network
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

